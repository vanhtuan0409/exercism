fn is_surround(center: (usize, usize), p: (usize, usize)) -> bool {
    center != p
        && (center.0 as i32 - p.0 as i32).abs() <= 1
        && (center.1 as i32 - p.1 as i32).abs() <= 1
}

fn count_mines_around(matrix: &[&str], point: (usize, usize)) -> u32 {
    let mut count = 0;
    let row_length = matrix.get(0).map(|row| row.len()).unwrap_or(0);
    for i in 0..matrix.len() {
        for j in 0..row_length {
            if is_surround(point, (i, j)) {
                count += match matrix.get(i).unwrap().chars().nth(j).unwrap() {
                    '*' => 1,
                    _ => 0,
                }
            }
        }
    }
    count
}

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let mut ret = vec![];
    for (row_index, row) in minefield.iter().enumerate() {
        let mut transformed_row = String::new();
        for (col_index, ch) in row.chars().enumerate() {
            let transformed_char = match ch {
                '*' => "*".to_string(),
                _ => match count_mines_around(minefield, (row_index, col_index)) {
                    0 => " ".to_string(),
                    x => x.to_string(),
                },
            };
            transformed_row.push_str(&transformed_char);
        }
        ret.push(transformed_row);
    }
    ret
}
