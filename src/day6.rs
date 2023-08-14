use std::collections::{HashSet, VecDeque};

const NECESSARY_DISTINCT_CHARS: usize = 14;

fn main() {
    const CONTENTS: &str = include_str!("day6.input");

    let mut vec = VecDeque::from_iter(CONTENTS.chars().take(NECESSARY_DISTINCT_CHARS - 1));
    let mut set = HashSet::<char>::with_capacity(NECESSARY_DISTINCT_CHARS);

    for (i, c) in CONTENTS.char_indices().skip(NECESSARY_DISTINCT_CHARS - 1) {
        vec.push_back(c);

        set.extend(vec.iter());
        if set.len() == NECESSARY_DISTINCT_CHARS {
            println!("{}", i + 1);
            break;
        }
        set.clear();

        vec.pop_front();
    }
}
