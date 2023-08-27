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

    fn move_towards(&self, other: Location) -> Self {
        Self {
            x: self.x + (other.x - self.x).signum(),
            y: self.y + (other.y - self.y).signum(),
        }
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
            steps: words.next().unwrap().parse::<u32>().unwrap(),
        }
    });

    let mut visited = HashSet::new();
    let mut rope = vec![Location { x: 0, y: 0 }; 10];

    for command in commands {
        for _ in 0..command.steps {
            rope[0] += command.direction;

            for i in 1..rope.len() {
                if !rope[i - 1].is_adjacent(&rope[i]) {
                    rope[i] = rope[i].move_towards(rope[i - 1]);
                }
            }

            visited.insert(rope[9]);
        }
    }

    println!("Visited {}", visited.len());
}
