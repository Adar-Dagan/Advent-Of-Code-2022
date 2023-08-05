fn main() {
    let contents = include_str!("day1.input");

    let elves = contents.split("\n\n");

    let mut max = [0, 0, 0];

    for elf in elves {
        let mut total = 0;

        for item in elf.split("\n") {
            total += item.parse::<i64>().unwrap();
        }

        match total {
            t if max[0] < t => {
                max[2] = max[1];
                max[1] = max[0];
                max[0] = total;
            }
            t if max[1] < t => {
                max[2] = max[1];
                max[1] = total;
            }
            t if max[2] < t => {
                max[2] = total;
            }
            _ => {}
        }
    }

    let sum: i64 = max.iter().sum();
    println!("max: {sum}");
}
