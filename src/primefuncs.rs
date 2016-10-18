//! Functions, such as factorisation and similar computations, which require use of prime numbers
//! to be calculated.

use segment;
use sieve::Sieve;

impl Sieve {
    /// Uses trial division to determine if the given number is prime.
    fn trial_division(&self, n: u64) -> bool {
        for p in self.iter() {
            if p.saturating_mul(p) > n {
                return true;
            } else if n % p == 0 {
                return false;
            }
        }

        true
    }

    /// Returns whether or not `n` is a prime number.
    ///
    /// Uses a simple lookup if `n` is not greater than the largest number known about by the
    /// sieve, and uses trial division otherwise.
    ///
    /// # Panics
    ///
    /// If `n` is larger than the square of the largest number known about by the sieve, this
    /// function will panic.
    ///
    /// # Examples
    ///
    /// ```
    /// let sieve = primesieve::Sieve::to_limit(100);
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
                } else if n <= self.limit().saturating_mul(self.limit()) {
                    true
                } else {
                    panic!("Too large to test for primality")
                }
            }
        }
    }
}