use std::cmp::Ordering;
use std::iter;

#[cfg(test)]
mod tests {
    mod add {
        use crate::add;

        #[test]
        fn returns_correct_result() {
            let n = vec![2, 5];
            let addend1 = vec![1, 3];
            let addend2 = vec![9, 8];

            assert_eq!(add(&n, &addend1), vec![3, 8]);
            assert_eq!(add(&n, &addend2), vec![1, 2, 3]);
        }
    }
}

fn add(n: &[u8], addend: &[u8]) -> Vec<u8> {
    let top = match n.len().cmp(&addend.len()) {
        Ordering::Greater => n,
        Ordering::Less => addend,
        Ordering::Equal => n,
    };

    let bottom = match n.len().cmp(&addend.len()) {
        Ordering::Greater => addend,
        Ordering::Less => n,
        Ordering::Equal => addend,
    };

    let length_diff = top.len() - bottom.len();
    let mut top_iter = top.iter().rev();
    let mut bottom_iter = bottom
        .iter()
        .rev()
        .chain(iter::repeat(&0u8).take(length_diff));
    let mut carry_digit = 0;
    let mut result: Vec<u8> = vec![];

    for _ in 0..top_iter.len() {
        let top_digit = top_iter.next().unwrap();
        let bottom_digit = bottom_iter.next().unwrap();

        let result = top_digit + bottom_digit + carry_digit;
        let result_digit = result % 10;
        carry_digit = result / 10;

        result.push(result_digit);
    }

    if carry_digit != 0 {
        result.push(carry_digit);
    }

    result
        .iter()
        .rev()
        .map(|&d| d)
        .collect()
}

fn main() {
    println!("{:?}", add(&[5, 8, 6], &[1, 3, 5, 7]));
}
