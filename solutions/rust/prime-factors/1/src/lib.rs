pub fn factors(mut n: u64) -> Vec<u64> {
    let mut primes = Vec::new();
    let mut candidate = 2;

    // We only need to check up to the square root of n
    while candidate * candidate <= n {
        while n % candidate == 0 {
            primes.push(candidate);
            n /= candidate;
        }
        candidate += 1;
    }

    // If n > 1, the remaining n is a prime factor
    if n > 1 {
        primes.push(n);
    }

    primes
}
