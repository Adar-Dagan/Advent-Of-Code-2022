fn main() {
    const CONTENTS: &str = include_str!("day2.input");

    let rounds = CONTENTS.trim().split("\n");

    let total = rounds.fold(0, |acc, round| {
        let choices: Vec<&str> = round.split(" ").collect();

        let [opponent_choice_string, my_choice_string]: [&str; 2] = match choices[..] {
            [a, b] => [a, b],
            _ => panic!("Invalid input"),
        };

        let opponent_choice = get_choice(opponent_choice_string);
        let my_choice = get_choice(my_choice_string);

        let match_points = if my_choice == (opponent_choice + 1) % 3 {
            6
        } else if opponent_choice == (my_choice + 1) % 3 {
            0
        } else {
            3
        };

        acc + match_points + (my_choice + 1)
    });

    println!("Total: {total}");
}

enum Choice {
    Rock = 0,
    Paper = 1,
    Scissors = 2,
}

fn get_choice(choice: &str) -> usize {
    let choice_num = match choice {
        "A" | "X" => Choice::Rock,
        "B" | "Y" => Choice::Paper,
        "C" | "Z" => Choice::Scissors,
        _ => panic!("Invalid choice"),
    };
    choice_num as usize
}
