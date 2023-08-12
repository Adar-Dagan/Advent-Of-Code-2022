use std::collections::HashSet;

fn main() {
    const CONTENTS: &str = include_str!("day3.input");

    let rucksacks = CONTENTS.lines();

    let total = rucksacks.fold(0, |acc, rucksack| -> u32 {
        let (first_compartment_string, second_compartment_string) =
            rucksack.split_at(rucksack.len() / 2);

        let first_compartment = first_compartment_string.chars().collect::<HashSet<char>>();
        let second_compartment = second_compartment_string.chars().collect::<HashSet<char>>();

        let duplicates_sum = first_compartment.iter().fold(0, |acc, letter| {
            acc + if second_compartment.contains(letter) {
                let letter_case = if letter.is_lowercase() {
                    'a' as u32 - 1
                } else {
                    'A' as u32 - 27
                };

                *letter as u32 - letter_case
            } else {
                0
            }
        });

        acc + duplicates_sum
    });

    println!("Total: {total}");
}
