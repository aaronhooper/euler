/// Returns the `n`th prime number.
///
/// # Examples
/// ```
/// use ten_thousand_and_first_prime::nth_prime;
///
/// let sixth_prime = nth_prime(6);
/// assert_eq!(sixth_prime, 13);
/// ```
pub fn nth_prime(nth: i32) -> i32 {
    let mut this_nth = 0;
    let mut maybe_prime = 1;

    while this_nth < nth {
        maybe_prime += 1;

        if is_prime(maybe_prime) {
            this_nth += 1;
        }
    }

    let prime = maybe_prime;
    prime
}

/// Returns true if the given integer value is a prime number.
///
/// # Examples
/// ```
/// use ten_thousand_and_first_prime::is_prime;
///
/// assert_eq!(is_prime(17), true);
/// assert_eq!(is_prime(20), false);
/// ```
pub fn is_prime(n: i32) -> bool {
    if n < 2 {
        return false
    }

    let sqrt_n = (n as f64).sqrt().floor() as i32;

    for divisor in 2..=sqrt_n {
        if n % divisor == 0 {
            return false
        }
    }

    true
}
