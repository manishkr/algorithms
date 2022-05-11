pub fn int_to_roman(num: i32) -> String {
  let map = vec![(1000, "M"), (900, "CM"), (500, "D"), (400, "CD"), (100, "C"), (90, "XC"),
                 (50, "L"), (40, "XL"), (10, "X"), (9, "IX"), (5, "V"), (4, "IV"), (1, "I")];

  let mut index = 0usize;
  let mut output = String::new();
  let mut input = num;

  while input > 0 {
    if input >= map[index].0 {
      output += map[index].1;
      input -= map[index].0;
    } else {
      index += 1;
    }
  }
  return output;
}
