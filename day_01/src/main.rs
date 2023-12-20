use std::env;
use std::fs;
use std::collections::VecDeque;

const NUMBERS: [(&str, &str); 10] = [
  ("zero", "z0o"),
  ("one", "o1e"),
  ("two", "t2o"),
  ("three", "t3e"),
  ("four", "f4r"),
  ("five", "f5e"),
  ("six", "s6x"),
  ("seven", "s7n"),
  ("eight", "e8t"),
  ("nine", "n9e"),
];


fn main() {
  let args: Vec<String> = env::args().collect();
  let file_path = &args[1];
  let contents = fs::read_to_string(file_path).expect(&format!("Couldn't read {file_path}"));
  // println!("File: {contents}");

  let res: u32 = contents.split("\n").map(line_value).sum();
  println!("Result: {res}");
}


fn line_value(line: &str) -> u32 {
  let init = line.to_string();
  // println!("line: {init}");
  let mut digits: VecDeque<String> = NUMBERS.iter().fold(
    init,
    |acc, num| acc.replace(num.0, num.1)
  ).matches(|c| char::is_ascii_digit(&c)).map(|m| m.to_owned()).collect();
  // println!("  digits: {digits:?}");
  let res: Option<u32> = match (digits.pop_front(), digits.pop_back()) {
    (Some(t), Some(o)) => (t + &o).parse().ok(),
    (Some(t), None) => t.repeat(2).parse().ok(),
    _ => None,
  };
  // println!("  res: {res:?}");
  if let Some(value) = res {
    value
  } else {
    0
  }
}
