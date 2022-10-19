#[cfg(test)]
mod tests {
    use crate::lattice_paths;

    #[test]
    fn returns_correct_result() {
        assert_eq!(lattice_paths(1), 2);
        assert_eq!(lattice_paths(2), 6);
    }
}

fn lattice_paths(width: u64) -> u64 {
    (1..=width).fold(1, |acc, n| acc * (width + n) / n)
}

fn main() {
    println!("{}", lattice_paths(20));
}
