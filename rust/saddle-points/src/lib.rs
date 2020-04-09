pub fn find_saddle_points(rows: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let height = rows.len();
    if rows.is_empty() {
        return Vec::new();
    }

    let width = rows[0].len();

    let columns = (0..width)
        .map(|i| rows.iter().map(|row| row[i]).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let max_of_each_row: Vec<&u64> = rows
        .iter()
        .map(|row| row.iter().max().unwrap_or(&u64::min_value()))
        .collect();

    let min_of_each_col: Vec<&u64> = columns
        .iter()
        .map(|col| col.iter().min().unwrap_or(&u64::max_value()))
        .collect();

    let mut saddle_points: Vec<(usize, usize)> = Vec::new();

    for x in 0..height {
        for y in 0..width {
            if rows[x][y] == *max_of_each_row[x] && rows[x][y] == *min_of_each_col[y] {
                saddle_points.push((x, y));
            }
        }
    }

    saddle_points
}
