#[cfg(test)]
mod tests {
    use crate::lattice_paths;

    #[test]
    fn returns_correct_result() {
        assert_eq!(lattice_paths(1, 1), 2);
        assert_eq!(lattice_paths(2, 2), 6);
    }
}

fn lattice_paths(width: u32, height: u32) -> u32 {
    if width == 0 || height == 0 {
        return 1;
    }

    lattice_paths(width - 1, height) + lattice_paths(width, height - 1)
}

fn main() {
    println!("{}", lattice_paths(20, 20));
}
