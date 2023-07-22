use crate::arithmetic::data_structures::fraction::Fraction;

mod data_structures;

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

/// Egyptian fractions are a representation method for fractions that were used in ancient Egypt.
/// They represent any fraction as the sum of fractions with a numerator of 1.
/// The idea is to use a set of unit fractions (fractions with a numerator of 1) to express the
/// original fraction. However, it's important to note that all the unit fractions in the
/// representation must have different denominators. For example, 2/3 can be represented as
/// 1/2 + 1/6 using Egyptian fractions.
///
/// Any fraction can have infinitely many Egyptian fraction representations, and for fractions less
/// than 1, it can be easily proven through mathematical induction. For fractions greater than or
/// equal to 1, it can be shown that the harmonic series diverges, which guarantees the existence of
/// Egyptian fraction representations.
///
/// The following algorithm uses a greedy approach to find the Egyptian fraction representation for
/// fractions less than 1:
///
/// **Input:** A fraction less than 1.
///
/// **Output:** The Egyptian fraction representation.
///
/// **Algorithm (using Greedy approach):**
/// 1. Start with an empty list to store the unit fractions.
/// 2. While the given fraction is not equal to 0:
///    a. Find the largest unit fraction (1/x) that is less than or equal to the given fraction.
///    b. Add this unit fraction to the list.
///    c. Subtract the value of the unit fraction from the given fraction.
///    d. Repeat steps a to c until the given fraction becomes 0.
///
/// **Note:**
/// It's important to be cautious when applying this algorithm to fractions with large denominators.
/// For fractions like 4/13, the denominators of the unit fractions may grow rapidly, leading to
/// overflow issues during calculations. Therefore, when implementing this algorithm, one should
/// consider using appropriate data types or implementing strategies to handle potential overflow.
///
/// ---
///
/// **Reference:**
/// - [Egyptian fraction - Wikipedia](https://en.wikipedia.org/wiki/Egyptian_fraction)
/// - [【証明あり】単位分数分解のやり方を解説！単位分数の和は無限通りに表せる！](https://mathsuke.jp/unit-fraction-decomposition/)
/// - [古代エジプトではどのような方法で5個のパンを8人に分けていたか？　〜エジプト分数の考え方〜](https://note.com/design_mame/n/ne2da1d025dd3)
pub fn egyptian_fraction(numerator: u64, denominator: u64) -> Vec<Fraction> {
    let mut result = Vec::new();
    let mut numerator = numerator;
    let mut denominator = denominator;
    while numerator != 1 {
        let unit_denominator = denominator / numerator + 1;
        println!("{}/{}", 1, unit_denominator);
        result.push(Fraction::new(1, unit_denominator));
        numerator = numerator * unit_denominator - denominator;
        denominator = denominator * unit_denominator;
    }
    result.push(Fraction::new(1, denominator));
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sieve_of_eratosthenes() {
        assert_eq!(sieve_of_eratosthenes(10), vec![2, 3, 5, 7]);
        assert_eq!(sieve_of_eratosthenes(20), vec![2, 3, 5, 7, 11, 13, 17, 19]);
    }
}
