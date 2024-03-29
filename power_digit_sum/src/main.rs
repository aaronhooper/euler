#[cfg(test)]
mod tests {
    mod multiply {
        use crate::multiply;

        #[test]
        fn returns_correct_result() {
            let n1 = vec![9, 8];
            let n2 = vec![6, 5, 5, 3, 6];
            let multiplier = 2;

            assert_eq!(multiply(&n1, multiplier), vec![1, 9, 6]);
            assert_eq!(multiply(&n2, multiplier), vec![1, 3, 1, 0, 7, 2]);
        }
    }

    mod power_digit_sum {
        use crate::power_digit_sum;

        #[test]
        fn returns_correct_result() {
            assert_eq!(power_digit_sum(15), 26);
        }
    }
}

fn multiply(n: &[u8], multiplier: u8) -> Vec<u8> {
    let n = n.iter().rev();
    let mut result: Vec<u8> = Vec::new();
    let mut carry_digit = 0;

    for digit in n {
        let result_digit = (digit * multiplier + carry_digit) % 10;
        carry_digit = (digit * multiplier + carry_digit) / 10;
        result.push(result_digit);
    }

    if carry_digit != 0 {
        result.push(carry_digit);
    }

    result.iter().rev().map(|&d| d).collect()
}

fn power_digit_sum(power: u32) -> u32 {
    let mut total: Vec<u8> = vec![1];

    for _ in 0..power {
        total = multiply(&total, 2);
    }

    total.iter().map(|&d| d as u32).sum()
}

fn main() {
    println!("{}", power_digit_sum(1000));
}
