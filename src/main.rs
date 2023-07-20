use crate::sort::bubble_sort;

mod sort;
mod arithmetic;

fn main() {
    bubble_sort(&mut [1, 5, 2, 3, 4]);

    let primes = arithmetic::sieve_of_eratosthenes(100);
    println!("Primes: {:?}", primes);
}
