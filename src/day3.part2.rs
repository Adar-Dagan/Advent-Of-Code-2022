use std::collections::HashSet;
use std::iter::zip;

fn main() {
    const CONTENTS: &str = include_str!("day3.input");

    let rucksacks = CONTENTS.lines();

    let rucksack_groups = zip(
        rucksacks.clone().step_by(3),
        zip(
            rucksacks.clone().skip(1).step_by(3),
            rucksacks.skip(2).step_by(3),
        ),
    )
    .map(|(first, (second, third))| {
        let first_compartment = first.chars().collect::<HashSet<char>>();
        let second_compartment = second.chars().collect::<HashSet<char>>();
        let third_compartment = third.chars().collect::<HashSet<char>>();

        vec![first_compartment, second_compartment, third_compartment]
    });

    let total = rucksack_groups.fold(0, |acc, rucksack_group| -> u32 {
        let first_intersection = rucksack_group[0]
            .intersection(&rucksack_group[1])
            .copied()
            .collect::<HashSet<char>>();

        let second_intersection = first_intersection
            .intersection(&rucksack_group[2])
            .collect::<HashSet<_>>();

        let common_letter = if second_intersection.len() == 1 {
            second_intersection.iter().next().unwrap()
        } else {
            panic!("More than one common letter found");
        };

        let common_letter_value = {
            let letter_case = if common_letter.is_lowercase() {
                'a' as u32 - 1
            } else {
                'A' as u32 - 27
            };

            **common_letter as u32 - letter_case
        };

        acc + common_letter_value
    });

    println!("Total: {total}");
}
