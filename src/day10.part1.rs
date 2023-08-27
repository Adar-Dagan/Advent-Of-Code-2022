use std::collections::HashSet;

enum Command {
    Addx(i32),
    Noop,
}

struct CpuState {
    x: i32,
    cycles: i32,
    signal: i32,
    report_cycles: HashSet<i32>,
}

impl CpuState {
    fn new<I: IntoIterator<Item = i32>>(report_cycles: I) -> Self {
        Self {
            x: 1,
            cycles: 1,
            signal: 0,
            report_cycles: HashSet::from_iter(report_cycles),
        }
    }

    fn increment_cycles(&mut self) {
        if self.report_cycles.contains(&self.cycles) {
            self.signal += self.x * self.cycles;
        }
        self.cycles += 1;
    }
}

fn main() {
    const CONTENTS: &str = include_str!("day10.example.input");

    let commands = CONTENTS.lines().map(|line| {
        let mut words = line.split_whitespace();

        match words.next().unwrap() {
            "addx" => Command::Addx(words.next().unwrap().parse().unwrap()),
            "noop" => Command::Noop,
            _ => unreachable!(),
        }
    });

    let mut cpu_state = CpuState::new((0..6).map(|n| n * 40 + 20));

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

    println!("{}", cpu_state.signal);
}
