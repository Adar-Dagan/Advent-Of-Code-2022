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
    let mut visible_trees = vec![vec![0; matrix_len]; matrix_len];

    for i in 0..matrix_len {
        let mut max_row_from_left = -1;
        let mut max_row_from_right = -1;
        let mut max_column_from_top = -1;
        let mut max_column_from_bottom = -1;
        for j in 0..matrix_len {
            if matrix[i][j] > max_row_from_left {
                visible_trees[i][j] = 1;
                max_row_from_left = matrix[i][j];
            }
            if matrix[i][matrix_len - j - 1] > max_row_from_right {
                visible_trees[i][matrix_len - j - 1] = 1;
                max_row_from_right = matrix[i][matrix_len - j - 1];
            }
            if matrix[j][i] > max_column_from_top {
                visible_trees[j][i] = 1;
                max_column_from_top = matrix[j][i];
            }
            if matrix[matrix_len - j - 1][i] > max_column_from_bottom {
                visible_trees[matrix_len - j - 1][i] = 1;
                max_column_from_bottom = matrix[matrix_len - j - 1][i];
            }
        }
    }

    let total_visible: i32 = visible_trees
        .iter()
        .map(|row| row.iter().sum::<i32>())
        .sum();
    println!("{total_visible}");
}
