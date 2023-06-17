use core::num::{dec2flt::number, self};

struct Brackets {
    n_pairs: Iterator,
    pairs_list: Option<String>,
}

impl Iterator for Brackets {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.n_pairs.next() {
                Some(number) => {
                    match Brackets::valid_pairs(number) {
                        Some(br_string) => {
                            self.pairs_list = Some(br_string);
                            break;
                        },
                        None => continue,
                    }
                },
                None => {
                    self.pairs_list = None;
                    break;
                },
            }
        }
        self.pairs_list
    }
}

impl Brackets {
    fn new(level: usize) -> Brackets {
        let start: usize = 2.pow(level - 1) - 1;
        let end: usize = 2 * (1 - 4.pow(level - 1)) / (1 - 4);
        Brackets {
            n_pairs: start..=end,
            pairs_list: None,
        }
    }
    fn valid_pairs(mut number: usize) -> Option<String> {
        let mut count: usize = 0;
        let mut pairs: String = String::from("(");
        while number != 0 {
            if number & 1 == 1 {
                count += 1;
                pairs.push("(");
            } else {
                count -= 1;
                pairs.push(")");
            }
            number >> 1;
        }
        if count == 0 {
            pairs.push(")");
            Some(pairs)
        } else {
            None
        }
    }
}