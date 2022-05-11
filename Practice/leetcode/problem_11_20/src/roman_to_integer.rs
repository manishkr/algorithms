use std::collections::HashMap;

pub fn roman_to_int(s: String) -> i32 {
  let map = HashMap::from([('I', 1), ('V', 5), ('X', 10),
    ('L', 50), ('C', 100), ('D', 500), ('M', 1000)]);

  let mut output = 0;
  for ch in s.chars().rev() {
    let num = map.get(&ch).unwrap();
    if 4 * num < output {
      output -= num;
    } else {
      output += num;
    }
  }
  return output;
}
