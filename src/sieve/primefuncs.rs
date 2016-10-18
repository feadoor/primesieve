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

    /// Returns whether or not `n` is a prime number, or `None` if `n` is larger than the square
    /// of the largest prime held in the sieve.
    ///
    /// Uses a simple lookup if `n` is not greater than the largest number known about by the
    /// sieve, and uses trial division otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// let sieve = primesieve::Sieve::to_limit(100);
    ///
    /// assert_eq!(sieve.is_prime(0), Some(false));
    /// assert_eq!(sieve.is_prime(1), Some(false));
    /// assert_eq!(sieve.is_prime(2), Some(true));
    /// assert_eq!(sieve.is_prime(3), Some(true));
    /// assert_eq!(sieve.is_prime(4), Some(false));
    /// assert_eq!(sieve.is_prime(5), Some(true));
    ///
    /// assert_eq!(sieve.is_prime(491), Some(true));
    /// assert_eq!(sieve.is_prime(493), Some(false));
    /// assert_eq!(sieve.is_prime(495), Some(false));
    /// assert_eq!(sieve.is_prime(497), Some(false));
    /// assert_eq!(sieve.is_prime(499), Some(true));
    ///
    /// assert_eq!(sieve.is_prime(1000001), None);
    /// ```
    pub fn is_prime(&self, n: u64) -> Option<bool> {
        match n {
            2 | 3 | 5 => Some(true),
            _ => {
                if n < self.limit() {
                    Some(segment::get(&self.primes, n))
                } else if n <= self.limit().saturating_mul(self.limit()) {
                    Some(Sieve::trial_division(&self, n))
                } else {
                    None
                }
            }
        }
    }

    /// Factorises `n` into (prime, exponent) pairs. Returns Err(remainder, partial factorisation)
    /// if `n` cannot be fully factorised without sieving for more primes.
    ///
    /// If `x` is the largest number known about by the sieve, then any integer having at most one
    /// prime factor larger than `x` can be factorised. In particular, any number not greater than
    /// `x^2` can be factorised.
    ///
    /// # Examples
    ///
    /// ```
    /// let sieve = primesieve::Sieve::to_limit(100);
    ///
    /// assert_eq!(sieve.factorise(2), Ok(vec![(2, 1)]));
    /// assert_eq!(sieve.factorise(4), Ok(vec![(2, 2)]));
    /// assert_eq!(sieve.factorise(1 << 63), Ok(vec![(2, 63)]));
    ///
    /// assert_eq!(sieve.factorise(2 * 3), Ok(vec![(2, 1), (3, 1)]));
    /// assert_eq!(sieve.factorise(89 * 97), Ok(vec![(89, 1), (97, 1)]));
    /// assert_eq!(sieve.factorise(8 * 9 * 5), Ok(vec![(2, 3), (3, 2), (5, 1)]));
    ///
    /// assert_eq!(sieve.factorise(2 * 3 * 5 * 991), Ok(vec![(2, 1), (3, 1), (5, 1), (991, 1)]));
    /// assert_eq!(sieve.factorise(2 * 3 * 5 * 991 * 991),
    ///            Err((991 * 991, vec![(2, 1), (3, 1), (5, 1)])));
    /// ```
    pub fn factorise(&self, mut n: u64) -> Result<Vec<(u64, u64)>, (u64, Vec<(u64, u64)>)> {
        // Deal with small values of `n` as special cases.
        if n == 0 { return Err((0, vec![])) }
        if n == 1 { return Ok(vec![]) }

        // Somewhere to store the result.
        let mut factors = Vec::new();

        // Iterate over the primes held in the sieve, checking if they are divisors of `n`.
        for p in self.iter() {

            // Check if `p` is large enough that we can stop iterating.
            if p.saturating_mul(p) > n {
                break;
            }

            // Repeatedly divide `n` by `p` until it is no longer divisible.
            let mut count = 0;
            while n % p == 0 {
                n /= p;
                count += 1;
            }
            if count > 0 {
                factors.push((p, count));
            }
        }

        // If there are any leftovers, check if it is small enough that we can guarantee that it
        // is prime.
        if n != 1 {
            if self.limit().saturating_mul(self.limit()) < n {
                return Err((n, factors));
            } else {
                factors.push((n, 1));
            }
        }

        Ok(factors)
    }
}
