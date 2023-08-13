fn main() {
    const CONTENTS: &str = include_str!("day5.input");

    let (initial_arrangement, move_instructions) = CONTENTS.split_once("\n\n").unwrap();

    let mut initial_arrangement_lines = initial_arrangement.lines().rev();

    let number_of_stacks = initial_arrangement_lines
        .next()
        .unwrap()
        .split_whitespace()
        .count();

    let mut stacks: Vec<Vec<char>> = vec![Vec::new(); number_of_stacks];

    for line in initial_arrangement_lines {
        let chars = line.chars().collect::<Vec<_>>();
        for i in 0..number_of_stacks {
            if chars[i * 4 + 1] != ' ' {
                stacks[i].push(chars[i * 4 + 1]);
            }
        }
    }

    move_instructions.lines().for_each(|line| {
        let (number_to_move, from, to) = parse_move_line(line);

        let from_length = stacks[from].len();
        let mut drain = stacks[from]
            .drain(from_length - number_to_move..)
            .collect::<Vec<_>>();
        stacks[to].append(&mut drain);
    });

    stacks.iter().for_each(|stack| {
        let last_crate = stack.last().unwrap();

        print!("{last_crate}");
    });
}

fn parse_move_line(line: &str) -> (usize, usize, usize) {
    let mut line_iterator = line
        .split_whitespace()
        .skip(1)
        .step_by(2)
        .map(|s| s.parse::<usize>().unwrap());

    (
        line_iterator.next().unwrap(),
        line_iterator.next().unwrap() - 1,
        line_iterator.next().unwrap() - 1,
    )
}
