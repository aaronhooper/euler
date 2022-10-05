struct Natural {
    n: u32,
}

impl Natural {
    fn sequence() -> Natural {
        Natural {
            n: 0,
        }
    }
}

impl Iterator for Natural {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.n = self.n.checked_add(1)?;
        Some(self.n)
    }
}

fn evenly_divisible(up_to: u32) -> Option<u32> {
    Natural::sequence()
        .find(|n| (1..=up_to).all(|i| n % i == 0))
}

#[cfg(test)]
mod tests {
    mod evenly_divisible {
        use crate::evenly_divisible;

        #[test]
        fn returns_first_evenly_divisible_number_up_to_10() {
            let result = evenly_divisible(10);
            let expected = Some(2520);

            assert_eq!(result, expected);
        }
    }

    mod natural {
        use crate::Natural;

        #[test]
        fn iterates_correctly() {
            let mut sequence = Natural::sequence();
            sequence.next();

            assert_eq!(sequence.next(), Some(2));
            assert_eq!(sequence.next(), Some(3));
        }
    }
}

fn main() {
    println!("{:?}", evenly_divisible(20));
}
