use std::env;
use std::fs;


fn main() {
  let args: Vec<String> = env::args().collect();
  let file_path = &args[1];
  let contents = fs::read_to_string(file_path).expect(&format!("Couldn't read {file_path}"));
  // println!("File: {contents}");

  let res: u32 = contents.split("\n").map(line_value).sum();
  println!("Result: {res}");
}


fn line_value(line: &str) -> u32 {
  let mut digits = line.match_indices(|c| char::is_ascii_digit(&c));
  let res: Option<u32> = match (digits.next(), digits.next_back()) {
    (Some(t), Some(o)) => (t.1.to_owned() + o.1).parse().ok(),
    (Some(t), None) => (t.1.to_owned() + t.1).parse().ok(),
    _ => None,
  };
  if let Some(value) = res {
    value
  } else {
    0
  }
}
