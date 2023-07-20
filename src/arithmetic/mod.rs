pub fn sieve_of_eratosthenes(n: usize) -> Vec<usize> {
    let mut primes = vec![true; n + 1];
    primes[0] = false;
    primes[1] = false;
    let mut i = 2;
    while i * i <= n {
        if primes[i] {
            let mut j = i * i;
            while j <= n {
                primes[j] = false;
                j += i;
            }
        }
        i += 1;
    }
    primes
        .iter()
        .enumerate()
        .filter_map(|(i, &is_prime)| if is_prime { Some(i) } else { None })
        .collect()
}
