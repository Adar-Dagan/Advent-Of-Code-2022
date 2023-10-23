use std::{collections::BTreeMap, ops::RangeInclusive};

fn get_range(first: i32, second: i32) -> RangeInclusive<i32> {
    if first < second {
        first..=second
    } else {
        second..=first
    }
}

#[derive(PartialEq)]
enum Content {
    Sand,
    Rock
}

struct Map {
    map: BTreeMap<i32, BTreeMap<i32, Content>>,
    floor: i32
}

impl Map {
    fn new() -> Self {
        Self {
            map: BTreeMap::new(),
            floor: 0
        }
    }

    fn insert(&mut self, x: i32, y: i32, content: Content) {
        if y + 2 > self.floor && content == Content::Rock {
            self.floor = y + 2;
        }

        if !self.map.contains_key(&x) {
            self.map.insert(x, BTreeMap::new());
        }
        self.map.get_mut(&x).unwrap().insert(y, content);
    }

    fn insert_rock_range(&mut self, first: (i32, i32), second: (i32, i32)) {
        let (x1, y1) = first;
        let (x2, y2) = second;


        if x1 == x2 {
            for y in get_range(y1, y2) {
                self.insert(x1, y, Content::Rock);
            }
        } else if y1 == y2 {
            for x in get_range(x1, x2) {
                self.insert(x, y1, Content::Rock);
            }
        } else {
            panic!("Not a straight line");
        }
    }

    fn next_down(&self, x: i32, y: i32) -> i32 {
        if y >= self.floor {
            return self.floor;
        }

        self.map.get(&x).and_then(|y_map| {
            y_map.range(y..).next()
        }).map(|(y, _)| *y).unwrap_or(self.floor)
    }

    fn get(&self, x: i32, y: i32) -> Option<&Content> {
        if y >= self.floor {
            return Some(&Content::Rock);
        }

        self.map.get(&x).and_then(|y_map| {
            y_map.get(&y)
        })
    }

    fn simulate_sand(&self) -> (i32, i32) {
        let mut sand = (500, 0);

        loop {
            let next_down = self.next_down(sand.0, sand.1);

            if self.get(sand.0 - 1, next_down).is_none() {
                sand = (sand.0 - 1, next_down);
            } else if self.get(sand.0 + 1, next_down).is_none() {
                sand = (sand.0 + 1, next_down);
            } else {
                return (sand.0, next_down - 1);
            }
        }
    }
}

fn main() {
    const CONTENTS: &str = include_str!("day14.input");

    let mut map = Map::new();
    for line in CONTENTS.lines() {
        let mut walls_instructions = line.split(" -> ").map(|coor| {
            let (x, y) = coor.split_once(",").unwrap();
            (x.parse::<i32>().unwrap(), y.parse::<i32>().unwrap())
        });

        let (x, y) = walls_instructions.next().unwrap();

        let mut last = (x, y);
        for coor in walls_instructions {
            map.insert_rock_range(last, coor);
            last = coor;
        }
    }

    let mut c = 0;
    loop {
        let n = map.simulate_sand();
        if n == (500, 0) {
            break;
        }

        map.insert(n.0, n.1, Content::Sand);
        c += 1;
    }
    println!("{}", c + 1);
}
