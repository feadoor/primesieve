//! A structure which sieves for prime numbers and provides functions to iterate over the primes,
//! to get the nth prime and for querying whether a particular number is prime.

use iterator;
use segment;
use segsieve::segmented_sieve;

const MODULUS: u64 = 240;

enum SmallPrime {
    Two,
    Three,
    Five,
    None,
}

/// A function which calculates an upper bound for the nth prime, using the bounds given on
/// [Wikipedia](https://en.wikipedia.org/wiki/Prime_number_theorem#Approximations_for_the_nth_prime_number)
fn upper_bound(n: usize) -> u64 {
    match n {
        0...5 => 12,
        _ => {
            let f = n as f64;
            (f * (f.ln() + f.ln().ln())) as u64
        }
    }
}

/// A structure which sieves for primes up to a given limit and stores the results for later
/// iteration and querying.
pub struct Sieve {
    /// The internal representation of the primes held in this sieve.
    primes: Vec<u64>,
    /// Intermediate counts of the number of primes up to a particular point.
    counts: Vec<usize>,
}

impl<'a> Sieve {
    /// Create a new `Sieve` which knows about the primes up to the given limit.
    pub fn to_limit(limit: u64) -> Sieve {
        // Sieve for primes using a segmented sieve.
        let sieve = segmented_sieve(limit);

        // Count the number of primes up to intermediate points in the sieve.
        let mut counts = Vec::with_capacity(sieve.len());
        let mut count = 0;
        for num in &sieve {
            count += num.count_ones() as usize;
            counts.push(count);
        }

        Sieve {
            primes: sieve,
            counts: counts,
        }
    }

    /// Create a new `Sieve` which knows about at least the first `n` primes.
    pub fn to_n_primes(n: usize) -> Sieve {
        // Get an upper bound on the `n`th prime and sieve for primes up to that limit using a
        // segmented sieve.
        let sieve = segmented_sieve(upper_bound(n + 1));

        // Count the number of primes up to intermediate points in the sieve.
        let mut counts = Vec::with_capacity(sieve.len());
        let mut count = 0;
        for num in &sieve {
            count += num.count_ones() as usize;
            counts.push(count);
        }

        Sieve {
            primes: sieve,
            counts: counts,
        }
    }

    /// Returns the highest number that this `Sieve` knows about. Note that this may be slightly
    /// larger than the limit the sieve was created with.
    ///
    /// # Examples
    ///
    /// ```
    /// let sieve = primesieve::Sieve::to_limit(1000);
    /// assert!(sieve.limit() >= 1000);
    /// ```
    pub fn limit(&self) -> u64 {
        MODULUS * self.primes.len() as u64
    }

    /// Returns the number of primes that this `Sieve` knows about. Note that this may be slightly
    /// higher than the number of primes the sieve was created with.
    ///
    /// # Examples
    ///
    /// ```
    /// let sieve = primesieve::Sieve::to_n_primes(1000);
    /// assert!(sieve.num_primes() >= 1000);
    /// ```
    pub fn num_primes(&self) -> usize {
        self.counts[self.counts.len() - 1]
    }

    /// Return an iterator over the primes in this `Sieve`.
    ///
    /// # Examples
    ///
    /// ```
    /// let sieve = primesieve::Sieve::to_limit(100);
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
    /// let sieve = primesieve::Sieve::to_limit(500);
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
            _ => {
                if n < self.limit() {
                    segment::get(&self.primes, n)
                } else {
                    panic!("Sieve limit exceeded")
                }
            }
        }
    }

    /// Returns the `n`th prime number, indexed from 0.
    ///
    /// # Panics
    ///
    /// If fewer than `n` prime numbers are held in the sieve, this function will panic.
    ///
    /// # Examples
    ///
    /// ```
    /// let sieve = primesieve::Sieve::to_n_primes(100);
    ///
    /// assert_eq!(sieve.nth_prime(0), 2);
    /// assert_eq!(sieve.nth_prime(1), 3);
    /// assert_eq!(sieve.nth_prime(2), 5);
    /// assert_eq!(sieve.nth_prime(3), 7);
    /// assert_eq!(sieve.nth_prime(4), 11);
    ///
    /// assert_eq!(sieve.nth_prime(97), 521);
    /// assert_eq!(sieve.nth_prime(98), 523);
    /// assert_eq!(sieve.nth_prime(99), 541);
    /// ```
    pub fn nth_prime(&self, n: usize) -> u64 {
        // If n is small enough (i.e. 0, 1 or 2) then return the prime directly. Otherwise, we
        // should do a binary search of `self.counts` to find the right prime.
        match n {
            0 => 2,
            1 => 3,
            2 => 5,
            _ => {
                if n < self.num_primes() {
                    // Find the index into `self.primes` where we will find the `n`th prime,
                    // remembering that the stored counts are offset by 3.
                    let k = n - 3;
                    let idx = match self.counts.binary_search(&k) {
                        Err(x) => x,
                        Ok(mut x) => {
                            while self.counts[x] == k {
                                x += 1;
                            }
                            x
                        }
                    };

                    // Now find the specific prime within this chunk of primes.
                    let count = self.counts[idx] - self.primes[idx].count_ones() as usize;
                    let primes = &[self.primes[idx]];
                    let mut primes_iter = iterator::SieveIterator::new(primes);
                    MODULUS * idx as u64 + primes_iter.nth((k - count) as usize).unwrap()
                } else {
                    panic!("Sieve limit exceeded")
                }
            }
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