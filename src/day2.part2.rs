enum Choice {
    Rock = 0,
    Paper = 1,
    Scissors = 2,
}

fn main() {
    const CONTENTS: &str = include_str!("day2.input");

    let rounds = CONTENTS.trim().split("\n");

    let total = rounds.fold(0, |acc, round| {
        let choices: Vec<&str> = round.split(" ").collect();

        let [opponent_choice_string, game_outcome]: [&str; 2] = match choices[..] {
            [a, b] => [a, b],
            _ => panic!("Invalid input"),
        };

        let opponent_choice = match opponent_choice_string {
            "A" => Choice::Rock,
            "B" => Choice::Paper,
            "C" => Choice::Scissors,
            _ => panic!("Invalid choice"),
        } as isize;

        let game_score = match game_outcome {
            "X" => 0 + ((opponent_choice - 1).rem_euclid(3) + 1),
            "Y" => 3 + (opponent_choice + 1),
            "Z" => 6 + ((opponent_choice + 1).rem_euclid(3) + 1),
            _ => panic!("Invalid choice"),
        };

        acc + game_score
    });

    println!("Total: {total}");
}
