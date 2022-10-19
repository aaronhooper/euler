#[cfg(test)]
mod tests {
    use crate::lattice_paths;

    #[test]
    fn returns_correct_result() {
        assert_eq!(lattice_paths(1), 2);
        assert_eq!(lattice_paths(2), 6);
    }
}

fn factorial(n: u64) -> u64 {
    if n == 0 {
        return 1;
    }

    n * factorial(n - 1)
}

fn lattice_paths(width: u64) -> u64 {
    let n = width * 2;
    let k = width;

    factorial(n) / (factorial(k) * factorial(n - k))
}

fn main() {
    println!("{}", lattice_paths(20));
}
