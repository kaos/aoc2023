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

  let calc = possible_games(&cubes);
  let res: u32 = contents.split("\n")
    .map(parse_game)
    .filter_map(calc)
    .sum();
  println!("Result 1: {res:?}");

  let calc = game_power;
  let res: u32 = contents.split("\n")
    .map(parse_game)
    .filter_map(calc)
    .sum();
  println!("Result 2: {res:?}");
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


fn possible_games(cubes: &HashMap::<String, u32>) -> impl Fn((u32, Vec<(u32, String)>)) -> Option<u32> + '_ {
  |(game_id, shown_cubes)| {
    if shown_cubes.iter().all(|(count, color): &(u32, String)| { count <= &cubes[color] }) {
      Some(game_id)
    } else {
      None
    }
  }
}


fn game_power((_, shown_cubes): (u32, Vec<(u32, String)>)) -> Option<u32> {
  let mut stats = HashMap::<String, u32>::from([("red".to_owned(), 0), ("green".to_owned(), 0), ("blue".to_owned(), 0)]);
  for (count, color) in shown_cubes.iter() {
    if count > &stats[color] {
      stats.insert(color.to_string(), *count);
    }
  }

  Some(stats.values().fold(1, |acc, value| acc * value))
}
