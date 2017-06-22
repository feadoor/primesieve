var searchIndex = {};
searchIndex["primesieve"] = {"doc":"A library for generating prime numbers using a segmented sieve.","items":[[3,"Sieve","primesieve","A structure which sieves for primes up to a given limit and stores the results for later iteration and querying.",null,null],[3,"SieveIterator","","A structure capable of iterating over the primes held in a `Sieve`.",null,null],[11,"is_prime","","Returns whether or not `n` is a prime number, or `Err(())` if `n` is larger than the square of the largest prime held in the sieve.",0,{"inputs":[{"name":"self"},{"name":"u64"}],"output":{"name":"result"}}],[11,"factorise","","Factorises `n` into (prime, exponent) pairs.",0,{"inputs":[{"name":"self"},{"name":"u64"}],"output":{"name":"result"}}],[11,"euler_phi","","Calculates the value of Euler's totient function `ϕ` at `n`.",0,{"inputs":[{"name":"self"},{"name":"u64"}],"output":{"name":"result"}}],[11,"number_of_divisors","","Calculates the number of divisors of `n`.",0,{"inputs":[{"name":"self"},{"name":"u64"}],"output":{"name":"result"}}],[11,"to_limit","","Create a new `Sieve` which knows about the primes up to the given limit.",0,{"inputs":[{"name":"u64"}],"output":{"name":"sieve"}}],[11,"to_n_primes","","Create a new `Sieve` which knows about at least the first `n` primes.",0,{"inputs":[{"name":"usize"}],"output":{"name":"sieve"}}],[11,"limit","","Returns the highest number that this `Sieve` knows about. Note that this may be slightly larger than the limit the sieve was created with.",0,{"inputs":[{"name":"self"}],"output":{"name":"u64"}}],[11,"num_primes","","Returns the number of primes that this `Sieve` knows about. Note that this may be slightly higher than the number of primes the sieve was created with.",0,{"inputs":[{"name":"self"}],"output":{"name":"usize"}}],[11,"nth_prime","","Returns the `n`th prime number, indexed from 0, or `None` if fewer than `n` prime numbers are held in the sieve.",0,{"inputs":[{"name":"self"},{"name":"usize"}],"output":{"name":"option"}}],[11,"iter","","Return an iterator over the primes in this `Sieve`.",0,{"inputs":[{"name":"self"}],"output":{"name":"sieveiterator"}}],[11,"next","","",1,{"inputs":[{"name":"self"}],"output":{"name":"option"}}]],"paths":[[3,"Sieve"],[3,"SieveIterator"]]};
initSearch(searchIndex);