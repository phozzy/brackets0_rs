struct Brackets {
    n_pairs: Box<dyn Iterator<Item = isize>>,
}

impl Iterator for Brackets {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let mut pair_list: Option<String> = None;
        loop {
            match self.n_pairs.next() {
                Some(number) => {
                    match Brackets::valid_pairs(number) {
                        Some(br_string) => {
                            pair_list = Some(br_string);
                            break;
                        },
                        None => continue,
                    }
                },
                None => {
                    break;
                },
            }
        }
        pair_list
    }
}

impl Brackets {
    fn new(level: u32) -> Brackets {
        let start: isize = isize::pow(2, level - 1) - 1;
        let end: isize = 2 * (1 - isize::pow(4, level - 1)) / (1 - 4);
        Brackets {
            n_pairs: Box::new(start..=end),
        }
    }
    fn valid_pairs(number: isize) -> Option<String> {
        let mut count: usize = 0;
        let mut pairs: String = String::from("(");
        while number != 0 {
            if number & 1 == 1 {
                count += 1;
                pairs.push('(');
            } else {
                count -= 1;
                pairs.push(')');
            }
            let _ = number >> 1;
        }
        if count == 0 {
            pairs.push(')');
            Some(pairs)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_pair() {
        let mut brackets = Brackets::new(1);
        assert_eq!(brackets.next(), Some(String::from("()")));
        assert_eq!(brackets.next(), None);
    }
}