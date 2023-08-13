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

        for _ in 0..number_to_move {
            let crate_to_move = stacks[from].pop().unwrap();
            stacks[to].push(crate_to_move);
        }
    });

    stacks.iter().for_each(|stack| {
        let last_crate = stack.last().unwrap();

        print!("{last_crate}");
    });
}

fn parse_move_line(line: &str) -> (usize, usize, usize) {
    let mut line_iterator = line.split_whitespace().skip(1);

    let number_to_move = line_iterator.next().unwrap().parse::<usize>().unwrap();

    line_iterator.next();

    let from = line_iterator.next().unwrap().parse::<usize>().unwrap() - 1;

    line_iterator.next();

    let to = line_iterator.next().unwrap().parse::<usize>().unwrap() - 1;

    (number_to_move, from, to)
}
