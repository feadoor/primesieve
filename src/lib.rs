//! A library for generating prime numbers using a segmented sieve.

mod iterator;
mod segment;
mod sieve;
mod wheel;

use sieve::segmented_sieve;

const MODULUS: u64 = 240;

enum SmallPrime {
    Two,
    Three,
    Five,
    None,
}

/// A structure which sieves for primes up to a given limit and stores the results for later
/// iteration and querying.
pub struct Sieve {
    /// The internal representation of the primes held in this sieve.
    primes: Vec<u64>,
}

impl<'a> Sieve {
    /// Create a new `Sieve` which knows about the primes up to the given limit.
    pub fn new(limit: u64) -> Sieve {
        Sieve { primes: segmented_sieve(limit) }
    }

    /// Returns the highest number that this `Sieve` knows about. Note that this may be slightly
    /// larger than the limit the sieve was created with.
    ///
    /// # Examples
    ///
    /// ```
    /// let sieve = primesieve::Sieve::new(1000);
    /// assert!(sieve.limit() >= 1000);
    /// ```
    pub fn limit(&self) -> u64 {
        return MODULUS * self.primes.len() as u64;
    }

    /// Return an iterator over the primes in this `Sieve`.
    ///
    /// # Examples
    ///
    /// ```
    /// let sieve = primesieve::Sieve::new(100);
    /// assert_eq!(sieve.iter().take_while(|&x| x < 100).collect::<Vec<u64>>(),
    ///            vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41,
    ///                 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97]);
    /// ```
    pub fn iter(&'a self) -> SieveIterator<'a> {
        SieveIterator {
            small: SmallPrime::Two,
            sieve_iter: iterator::SieveIterator::new(&self.primes),
        }
    }

    /// Returns whether or not `n` is a prime number.
    ///
    /// # Panics
    ///
    /// If `n` is out of range for the sieve, this function will panic.
    ///
    /// # Examples
    ///
    /// ```
    /// let sieve = primesieve::Sieve::new(500);
    ///
    /// assert_eq!(sieve.is_prime(0), false);
    /// assert_eq!(sieve.is_prime(1), false);
    /// assert_eq!(sieve.is_prime(2), true);
    /// assert_eq!(sieve.is_prime(3), true);
    /// assert_eq!(sieve.is_prime(4), false);
    /// assert_eq!(sieve.is_prime(5), true);
    ///
    /// assert_eq!(sieve.is_prime(491), true);
    /// assert_eq!(sieve.is_prime(493), false);
    /// assert_eq!(sieve.is_prime(495), false);
    /// assert_eq!(sieve.is_prime(497), false);
    /// assert_eq!(sieve.is_prime(499), true);
    /// ```
    pub fn is_prime(&self, n: u64) -> bool {
        match n {
            2 | 3 | 5 => true,
            _ => segment::get(&self.primes, n)
        }
    }
}

/// A structure capable of iterating over the primes held in a `Sieve`.
pub struct SieveIterator<'a> {
    /// The next small prime (2, 3 or 5) to yield.
    small: SmallPrime,
    /// An iterator over the primes encoded in the sieve.
    sieve_iter: iterator::SieveIterator<'a>,
}

impl<'a> Iterator for SieveIterator<'a> {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        // Yield a small prime if needed.
        match self.small {
            SmallPrime::Two => {
                self.small = SmallPrime::Three;
                return Some(2);
            }
            SmallPrime::Three => {
                self.small = SmallPrime::Five;
                return Some(3);
            }
            SmallPrime::Five => {
                self.small = SmallPrime::None;
                return Some(5);
            }
            SmallPrime::None => {}
        }

        // If all the small primes are out of the way, then start yielding from sieve_iter.
        self.sieve_iter.next()
    }
}