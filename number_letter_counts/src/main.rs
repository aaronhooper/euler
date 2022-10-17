#[cfg(test)]
mod tests {
    mod test_to_words {
        use crate::to_words;
        #[test]
        fn returns_words_of_ints_under_20() {
            let expected = vec![
                "zero", "one", "two", "three", "four",
                "five", "six", "seven", "eight", "nine",
                "ten", "eleven", "twelve", "thirteen", "fourteen",
                "fifteen", "sixteen", "seventeen", "eighteen", "nineteen",
            ];

            (0..20)
                .map(|n| to_words(n))
                .enumerate()
                .for_each(|(i, word)| assert_eq!(word, expected[i]));
        }

        #[test]
        fn returns_words_of_ints_under_100() {
            let expected = vec!["twenty", "thirty-one", "ninety-nine"];

            vec![20, 31, 99]
                .iter()
                .enumerate()
                .for_each(|(i, &n)| assert_eq!(to_words(n), expected[i]));
        }

        #[test]
        fn returns_words_of_ints_up_to_999() {
            let expected = vec![
                "one hundred",
                "two hundred and fifty",
                "five hundred and forty-six",
                "seven hundred and seventeen",
            ];

            vec![100, 250, 546, 717]
                .iter()
                .enumerate()
                .for_each(|(i, &n)| assert_eq!(to_words(n), expected[i]));
        }

        #[test]
        fn returns_multiples_of_100() {
            let expected = vec![
                "one hundred",
                "five hundred",
                "nine hundred",
            ];

            vec![100, 500, 900]
                .iter()
                .enumerate()
                .for_each(|(i, &n)| assert_eq!(to_words(n), expected[i]));
        }

        #[test]
        fn returns_word_for_1000() {
            assert_eq!(to_words(1000), "one thousand");
        }
    }
}

const NUMBER_WORDS: [&str; 30] = [
    "zero", "one", "two", "three", "four",
    "five", "six", "seven", "eight", "nine",
    "ten", "eleven", "twelve", "thirteen", "fourteen",
    "fifteen", "sixteen", "seventeen", "eighteen", "nineteen",
    "twenty", "thirty", "forty", "fifty",
    "sixty", "seventy", "eighty", "ninety",
    "hundred", "thousand",
];

fn to_words(n: u32) -> String {
    let mut n = n;
    let mut words: Vec<String> = Vec::new();
    let last_two_digits = n as usize % 100;
    let tens_offset = 18;
    let hundred_word = &NUMBER_WORDS[28];

    if n == 1000 {
        return "one thousand".to_string();
    }

    if n >= 100 {
        let hundreds_place = n as usize / 100;

        words.push(NUMBER_WORDS[hundreds_place].to_string());
        words.push(hundred_word.to_string());

        if last_two_digits == 0 {
            return words.join(" ");
        }
        else {
            words.push("and".to_string());
            n = n - hundreds_place as u32 * 100;
        }
    }

    let last_two_digits = n as usize % 100;

    if last_two_digits >= 20 {
        let tens_place = last_two_digits / 10;
        let ones_place = last_two_digits % 10;
        let mut word_part = String::from("");

        if ones_place != 0 {
            word_part.push_str(&NUMBER_WORDS[tens_place + tens_offset]);
            word_part.push_str("-");
            word_part.push_str(&NUMBER_WORDS[ones_place]);
        }
        else {
            word_part.push_str(&NUMBER_WORDS[tens_place + tens_offset]);
        }

        words.push(word_part);
    }
    else {
        words.push(NUMBER_WORDS[last_two_digits].to_string());
    }

    words.join(" ")
}

fn main() {
}
