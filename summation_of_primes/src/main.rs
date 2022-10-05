fn sieve_of_eratosthenes(n: u32) -> Vec<u32> {
    let n = n as usize;
    let mut primes = vec![true; n];
    let sqrt_n = (n as f64).sqrt().floor() as usize;

    primes[0] = false;
    primes[1] = false;

    for i in 2..=sqrt_n {
        if primes[i] == true {
            let mut j = i * i;

            while j < n {
                primes[j] = false;
                j = j + i;
            }
        }
    }

    primes.into_iter()
        .enumerate()
        .filter(|(_, is_prime)| *is_prime)
        .map(|(p, _)| p as u32)
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::sieve_of_eratosthenes;

    #[test]
    fn finds_correct_primes() {
        let expected = vec![2, 3, 5, 7];
        let result = sieve_of_eratosthenes(10);

        assert_eq!(result, expected);
    }
}

fn main() {
    let sum: u64 = sieve_of_eratosthenes(2_000_000)
        .into_iter()
        .map(|p| p as u64)
        .sum();

    println!("{}", sum);
}
