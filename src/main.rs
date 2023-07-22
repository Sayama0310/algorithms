use crate::sort::bubble_sort;

mod sort;
mod arithmetic;

fn main() {
    bubble_sort(&mut [1, 5, 2, 3, 4]);

    let primes = arithmetic::sieve_of_eratosthenes(100);
    println!("Primes: {:?}", primes);

    let egyptian_fraction = arithmetic::egyptian_fraction(4, 13);
    println!("Egyptian fraction: {}", egyptian_fraction
        .iter()
        .map(|fraction| format!("{}", fraction))
        .collect::<Vec<String>>()
        .join(", "));
}
