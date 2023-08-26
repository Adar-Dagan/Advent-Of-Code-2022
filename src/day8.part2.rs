fn main() {
    const CONTENTS: &str = include_str!("day8.input");

    let matrix = CONTENTS
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap() as i32)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let matrix_len = matrix.len();

    let mut max_visibility = 0;

    let go_left = |x, y| (x, y - 1);
    let go_right = |x, y| (x, y + 1);
    let go_up = |x, y| (x + 1, y);
    let go_down = |x, y| (x - 1, y);

    for i in 0..matrix_len {
        for j in 0..matrix_len {
            let visibilities = vec![
                find_visibility(&matrix, i, j, go_left),
                find_visibility(&matrix, i, j, go_right),
                find_visibility(&matrix, i, j, go_up),
                find_visibility(&matrix, i, j, go_down),
            ];
            let visibility = visibilities.into_iter().reduce(|a, b| a * b).unwrap();
            max_visibility = max_visibility.max(visibility);
        }
    }

    println!("Max visibility: {}", max_visibility);
}

fn find_visibility<F>(matrix: &Vec<Vec<i32>>, i: usize, j: usize, direction: F) -> i32
where
    F: Fn(i32, i32) -> (i32, i32),
{
    let (mut x, mut y) = direction(i as i32, j as i32);
    let value = &matrix[i][j];

    let mut visible = 0;
    while let Some(current_value) = matrix.get(x as usize).and_then(|row| row.get(y as usize)) {
        visible += 1;
        if current_value >= value {
            break;
        }
        (x, y) = direction(x, y);
    }
    visible
}
