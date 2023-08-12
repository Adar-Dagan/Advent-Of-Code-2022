#[derive(Copy, Clone)]
struct Elf {
    start: usize,
    end: usize,
}

impl Elf {
    pub fn new(s: &str) -> Self {
        let (start, end) = s.split_once('-').unwrap();
        Self {
            start: start.parse().unwrap(),
            end: end.parse().unwrap(),
        }
    }

    fn contains(&self, other: Elf) -> bool {
        self.start <= other.start && other.end <= self.end
    }

    fn overlaps(&self, other: Elf) -> bool {
        other.end >= self.start && self.end >= other.start
    }
}

fn main() {
    const CONTENTS: &str = include_str!("day4.input");

    let total = CONTENTS.lines().fold(0, |acc, pair| {
        let (first_elf_string, second_elf_string) = pair.split_once(',').unwrap();

        let first_elf = Elf::new(first_elf_string);
        let second_elf = Elf::new(second_elf_string);

        acc + if first_elf.overlaps(second_elf) { 1 } else { 0 }
    });

    println!("Total: {total}");
}
