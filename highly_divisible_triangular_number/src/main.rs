struct Triangular {
    i: u32,
    n: u32,
}

impl Triangular {
    fn sequence() -> Triangular {
        Triangular {
            i: 1,
            n: 0,
        }
    }
}

impl Iterator for Triangular {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.n = self.n.checked_add(self.i)?;
        self.i = self.i.checked_add(1)?;
        Some(self.n)
    }
}

fn gcd(a: u32, b: u32) -> u32 {
    if b == 0 {
        return a;
    }

    gcd(b, a % b)
}

fn main() {
    let divisible_by_over_500_numbers = |n: &u32| -> bool {
        let n = *n;
        let mut n_divisors = 2;
        let sqrt_n = (n as f64).sqrt().floor() as u32;

        for i in 2..=sqrt_n {
            if n % i == 0 {
                if n/i == i {
                    n_divisors += 1;
                } else {
                    n_divisors += 2;
                }

                if n_divisors > 500 {
                    return true;
                }
            }
        }

        false
    };

    let answer = Triangular::sequence().find(divisible_by_over_500_numbers);
    println!("{:?}", answer);
}
