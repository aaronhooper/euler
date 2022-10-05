fn main() {
    let sum: u32 = 1000;
    let mut product = 0;
    let mut done = false;

    for a in 1..sum {
        if done {
            break;
        }

        for b in 1..sum {
            if done {
                break;
            }

            for c in 1..sum {
                if done {
                    break;
                }

                let inequality_satisfied = a < b && b < c;

                if !inequality_satisfied {
                    continue;
                }

                let is_triplet = a.pow(2) + b.pow(2) == c.pow(2);

                if !is_triplet {
                    continue;
                }

                if a + b + c == sum {
                    product = a * b * c;
                    done = true;
                }
            }
        }
    }

    println!("{}", product);
}
