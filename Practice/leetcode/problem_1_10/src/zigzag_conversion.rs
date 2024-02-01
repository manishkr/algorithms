pub fn convert(s: String, num_rows: i32) -> String {
    if num_rows == 1 || s.len() < num_rows as usize {
        return s;
    }
    let mut rows: Vec<String> = vec![String::new(); s.len()];
    let mut reverse = false;
    let mut row = 0i32;
    let last_row = num_rows - 1;

    for i in 0..s.len() {
        rows[row as usize] += &s[i..=i];
        let inc = if reverse { -1i32 } else { 1i32 };
        row += inc;
        reverse = reverse ^ (row == last_row || row == 0);
    }

    return rows.into_iter().collect();
}
