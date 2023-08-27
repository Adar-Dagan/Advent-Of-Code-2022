enum Command {
    Addx(i32),
    Noop,
}

struct SystemState {
    x: i32,
    cycles: i32,
    screen: [[char; 40]; 6],
}

impl SystemState {
    fn new() -> Self {
        Self {
            x: 1,
            cycles: 0,
            screen: [['.'; 40]; 6],
        }
    }

    fn increment_cycles(&mut self) {
        let row = self.cycles / 40;
        let pixel = self.cycles % 40;
        if (pixel - self.x).abs() <= 1 {
            self.screen[row as usize][pixel as usize] = '#';
        }
        self.cycles += 1;
    }
}

fn main() {
    const CONTENTS: &str = include_str!("day10.input");

    let commands = CONTENTS.lines().map(|line| {
        let mut words = line.split_whitespace();

        match words.next().unwrap() {
            "addx" => Command::Addx(words.next().unwrap().parse().unwrap()),
            "noop" => Command::Noop,
            _ => unreachable!(),
        }
    });

    let mut cpu_state = SystemState::new();

    for command in commands {
        cpu_state.increment_cycles();

        match command {
            Command::Addx(n) => {
                cpu_state.increment_cycles();
                cpu_state.x += n;
            }
            Command::Noop => (),
        };
    }

    for row in cpu_state.screen.iter() {
        println!("{}", row.iter().collect::<String>());
    }
}
