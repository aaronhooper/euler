#[cfg(test)]
mod tests {
    mod collatz {
        use crate::collatz;

        #[test]
        fn passes_the_test() {
            let result = collatz(13);
            let expected = vec![13, 40, 20, 10, 5, 16, 8, 4, 2, 1];

            assert_eq!(result, expected);
        }
    }
}

fn collatz(start: u64) -> Vec<u64> {
    let mut curr = start;
    let mut sequence = vec![curr];

    while curr != 1 {
        if curr % 2 == 0 {
            curr = curr / 2;
        } else {
            curr = curr * 3 + 1;
        }

        sequence.push(curr);
    }

    sequence
}

fn longest_collatz(under: u64) -> Option<u64> {
    (1..under).max_by(|&a, &b| collatz(a).len().cmp(&collatz(b).len()))
}

fn main() {
    println!("{:?}", longest_collatz(1_000_000));
}
