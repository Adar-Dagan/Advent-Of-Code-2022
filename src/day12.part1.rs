fn main() {
    const CONTENT: &str = include_str!("day12.input");

    let start = CONTENT.find('S').unwrap();
    let end = CONTENT.find('E').unwrap();

    let map = CONTENT
        .lines()
        .map(|line| {
            line.as_bytes()
                .iter()
                .map(|&b| match b {
                    b'S' => b'a',
                    b'E' => b'z',
                    _ => b,
                } as i32)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let line_length = map[0].len() + 1;
    let (start_x, start_y) = (start / line_length, start % line_length);
    let end = (end / line_length, end % line_length);

    // BFS modified to count steps treating only valid steps as connected nodes

    let mut queue = vec![(start_x, start_y)];
    let mut next_queue = vec![];

    let mut visited = vec![vec![false; map[0].len()]; map.len()];
    visited[start_x][start_y] = true;

    let mut step = 0;

    loop {
        for (x, y) in queue.drain(0..) {
            if (x, y) == end {
                println!("Found exit in {} steps", step);
                return;
            }

            for (delta_x, delta_y) in [(-1, 0), (1, 0), (0, -1), (0, 1)].iter() {
                let (neighbor_x, neighbor_y) = (x as i32 + delta_x, y as i32 + delta_y);

                if neighbor_x < 0
                    || neighbor_y < 0
                    || neighbor_x >= map.len() as i32
                    || neighbor_y >= map[0].len() as i32
                {
                    continue;
                }
                let (neighbor_x, neighbor_y) = (neighbor_x as usize, neighbor_y as usize);

                let value_difference = map[neighbor_x][neighbor_y] - map[x][y];

                if !visited[neighbor_x][neighbor_y] && value_difference <= 1 {
                    next_queue.push((neighbor_x, neighbor_y));
                    visited[neighbor_x][neighbor_y] = true;
                }
            }
        }

        queue.append(&mut next_queue);
        step += 1;
    }
}
