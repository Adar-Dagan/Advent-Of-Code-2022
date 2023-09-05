use std::str::FromStr;

enum Content {
    List(List),
    Number(u32),
}

struct List {
    content: Vec<Content>,
}

impl List {
    fn compare_to(&self, other: &List) -> Option<bool> {
        let mut other_iter = other.content.iter();

        for self_item in self.content.iter() {
            let other_item = other_iter.next();
            if other_item.is_none() {
                return Some(false);
            }

            match (self_item, other_item.unwrap()) {
                (Content::Number(a), Content::Number(b)) => {
                    if a != b {
                        return Some(a < b);
                    }
                }
                (Content::Number(n), Content::List(l)) => {
                    let first_list = List {
                        content: vec![Content::Number(*n)],
                    };
                    if let Some(b) = first_list.compare_to(l) {
                        return Some(b);
                    }
                }
                (Content::List(l), Content::Number(n)) => {
                    let second_list = List {
                        content: vec![Content::Number(*n)],
                    };
                    if let Some(b) = l.compare_to(&second_list) {
                        return Some(b);
                    }
                }
                (Content::List(l1), Content::List(l2)) => {
                    if let Some(b) = l1.compare_to(l2) {
                        return Some(b);
                    }
                }
            }
        }

        return if other_iter.next().is_none() {
            None
        } else {
            Some(true)
        };
    }
}

impl FromStr for List {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut content = Vec::new();
        let mut depth = -1;
        let mut sublist_start = 0;
        let mut prev_number = None;
        for (i, c) in s.chars().enumerate() {
            match c {
                '[' => {
                    depth += 1;
                    if depth == 1 {
                        sublist_start = i;
                    }
                }
                ']' => {
                    depth -= 1;
                    if depth == 0 {
                        content.push(Content::List(List::from_str(&s[sublist_start..=i])?));
                    } else if depth == -1 {
                        if let Some(p) = prev_number {
                            content.push(Content::Number(p));
                            prev_number = None;
                        }
                    }
                }
                c if c.is_numeric() && depth == 0 => {
                    if let Some(p) = prev_number {
                        content.push(Content::Number(p * 10 + c.to_digit(10).unwrap()));
                        prev_number = None;
                    } else {
                        prev_number = c.to_digit(10);
                    }
                }
                ',' => {
                    if let Some(p) = prev_number {
                        content.push(Content::Number(p));
                        prev_number = None;
                    }
                }
                _ => {}
            }
        }
        return Ok(List { content });
    }
}

fn main() {
    const CONTENT: &str = include_str!("day13.input");

    let first = CONTENT
        .split("\n\n")
        .enumerate()
        .map(|(i, pair)| {
            let (first, second) = pair.split_once('\n').unwrap();
            let first = first.parse::<List>().unwrap();
            let second = second.parse::<List>().unwrap();

            let comparison = first.compare_to(&second);
            let comparison = comparison.is_none() || comparison.unwrap();
            return (i + 1, comparison);
        })
        .fold(0, |acc, (i, comparison)| {
            if comparison {
                return acc + i;
            }
            return acc;
        });

    println!("first: {}", first);
}
