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

    mod multiply {
        use crate::multiply;

        #[test]
        fn returns_correct_result() {
            let n1 = vec![9, 9, 9];
            let n2 = vec![9, 2];
            let multiplier1 = vec![8, 9];
            let multiplier2 = vec![2, 3, 9];

            assert_eq!(multiply(&n1, &n1), vec![9, 9, 8, 0, 0, 1]);
            assert_eq!(multiply(&n2, &multiplier1), vec![8, 1, 8, 8]);
            assert_eq!(multiply(&n2, &multiplier2), vec![2, 1, 9, 8, 8]);
        }
    }

    mod factorial {
        use crate::factorial;

        #[test]
        fn returns_correct_result() {
            assert_eq!(factorial(&[5]), vec![1, 2, 0]);
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

fn multiply(n: &[u8], multiplier: &[u8]) -> Vec<u8> {
    let top = match n.cmp(&multiplier) {
        Ordering::Greater => n,
        Ordering::Less => multiplier,
        Ordering::Equal => n,
    };

    let bottom = match n.cmp(&multiplier) {
        Ordering::Greater => multiplier,
        Ordering::Less => n,
        Ordering::Equal => multiplier,
    };

    let mut summands: Vec<Vec<u8>> = vec![];

    for (n_zeroes, bottom_digit) in bottom.iter().rev().enumerate() {
        let mut summand: Vec<u8> = iter::repeat(0u8).take(n_zeroes).collect();
        let mut carry_digit = 0;

        for top_digit in top.iter().rev() {
            let result = top_digit * bottom_digit + carry_digit;
            let result_digit = result % 10;
            carry_digit = result / 10;
            summand.push(result_digit);
        }

        if carry_digit != 0 {
            summand.push(carry_digit);
        }

        summand = summand.iter().rev().collect();
        summands.push(summand);
    }

    summands
        .into_iter()
        .reduce(|acc, summand| add(&acc, &summand))
        .unwrap()
}

fn factorial(n: &[u8]) -> Vec<u8> {
    let mut i = vec![0];
    let mut acc = vec![1];

    loop {
        if &i == n {
            break;
        }

        let i_plus_1 = add(&i, &[1]);
        acc = multiply(&acc, &i_plus_1);
        i = i_plus_1;
    }

    acc
}

fn main() {
    println!("{:?}", add(&[5, 8, 6], &[1, 3, 5, 7]));
}
