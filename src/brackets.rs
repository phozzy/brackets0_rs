pub struct Brackets {
    n_pairs: Box<dyn Iterator<Item = usize>>,
}

impl Iterator for Brackets {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let mut pair_list: Option<String> = None;
        while let Some(number) = self.n_pairs.next() {
            if let Some(br_string) = Brackets::valid_pairs(number) {
                pair_list = Some(br_string);
                break;
            }
        }
        pair_list
    }
}

impl Brackets {
    pub fn new(level: u32) -> Brackets {
        let start: usize = usize::pow(2, level - 1) - 1;
        let end: usize = 2 * (usize::pow(4, level - 1) - 1) / (4 - 1);
        Brackets {
            n_pairs: Box::new(start..=end),
        }
    }
    fn valid_pairs(mut number: usize) -> Option<String> {
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
            number >>= 1;
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