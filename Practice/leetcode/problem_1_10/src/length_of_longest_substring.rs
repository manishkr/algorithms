use std::cmp::max;
use std::collections::HashMap;

pub fn length_of_longest_substring(s: String) -> i32 {
    let mut char_map: HashMap<char, i32> = HashMap::new();
    let mut max_length = 0;
    let mut index = 0;
    for c in s.char_indices() {
        if let Some(value) = char_map.get(&c.1) {
            index = max(index, value.to_owned());
        }
        max_length = max(max_length, c.0 as i32 - index + 1);
        char_map.insert(c.1, c.0 as i32 + 1);
    }
    return max_length;
}
