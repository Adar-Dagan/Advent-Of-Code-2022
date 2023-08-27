use std::{collections::HashSet, ops::AddAssign};

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Location {
    x: i32,
    y: i32,
}

impl AddAssign for Location {
    fn add_assign(&mut self, other: Location) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl Location {
    fn is_adjacent(&self, other: &Location) -> bool {
        (self.x - other.x).abs() <= 1 && (self.y - other.y).abs() <= 1
    }
}

struct Command {
    direction: Location,
    steps: u32,
}

fn main() {
    const CONTENT: &str = include_str!("day9.input");

    let commands = CONTENT.lines().map(|line| {
        let mut words = line.split_whitespace();
        Command {
            direction: match words.next().unwrap() {
                "U" => Location { x: 0, y: 1 },
                "D" => Location { x: 0, y: -1 },
                "L" => Location { x: -1, y: 0 },
                "R" => Location { x: 1, y: 0 },
                _ => panic!("Unknown direction"),
            },
            steps: words
                .next()
                .and_then(|c| Some(c.parse::<u32>().unwrap()))
                .unwrap(),
        }
    });

    let mut visited = HashSet::new();
    let mut tail = Location { x: 0, y: 0 };
    let mut head = Location { x: 0, y: 0 };

    for command in commands {
        for _ in 0..command.steps {
            let prev_head = head;
            head += command.direction;
            if !head.is_adjacent(&tail) {
                tail = prev_head;
            }

            visited.insert(tail);
        }
    }

    println!("Visited {}", visited.len());
}
