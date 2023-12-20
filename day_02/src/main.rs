use std::env;
use std::fs;
use std::collections::HashMap;


fn main() {
  let args: Vec<String> = env::args().collect();
  let file_path = &args[1];
  let contents = fs::read_to_string(file_path).expect(&format!("Couldn't read {file_path}"));

  let cubes = HashMap::from([
    ("red".to_owned(), 12),
    ("green".to_owned(), 13),
    ("blue".to_owned(), 14),
  ]);

  let res: u32 = contents.split("\n").map(parse_game)
    .filter_map(|(game_id, shown_cubes): (u32, Vec<(u32, String)>)| {
      if shown_cubes.iter().all(|(count, color): &(u32, String)| count <= &cubes[color]) {
        Some(game_id)
      } else {
        None
      }
    })
    .sum();

  println!("Result: {res:?}");
}


fn parse_game(line: &str) -> (u32, Vec<(u32, String)>) {
  line.split(": ").fold((0, vec![]), |acc, v| {
    match v.starts_with("Game ") {
      true => (v[5..].parse::<u32>().unwrap(), acc.1),
      false => (acc.0, parse_cubes(&v.replace(";", ","))),
    }
  })
}


fn parse_cubes(game: &str) -> Vec<(u32, String)> {
  game.split(", ").filter_map(|cubes| {
    match cubes.split(" ").collect::<Vec<&str>>()[..] {
      [count, color] => Some((count.parse::<u32>().unwrap(), color.to_owned())),
      _ => None
    }
  }).collect()
}
