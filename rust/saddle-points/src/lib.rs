pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let mut ret = vec![];
    for (y, row) in input.iter().enumerate() {
        for (x, val) in row.iter().enumerate() {
            let is_row_largest = row.iter().all(|it| it <= val);
            let is_col_smallest = input.iter().map(|it| it[x]).all(|x| x >= *val);
            if is_row_largest && is_col_smallest {
                ret.push((y, x))
            }
        }
    }
    ret
}
