use std::collections::HashMap;

pub fn exec(input: &str) -> String {
  let game_values = HashMap::from([
    ("A", 1),
    ("B", 2),
    ("C", 3),
    ("X", 1),
    ("Y", 2),
    ("Z", 3),
  ]);

  let games = input
        .split("\n")
        .map(|x| {
          let mut g = x.split(' ').map(|g| game_values.get(g).unwrap());
          (
            (g.next()).unwrap(),
            (g.next()).unwrap(),
          )
        });

  let score1: i32 = games.clone().map(|x| {
          let a: i32 = x.0.clone();
          let b: i32 = x.1.clone();
          if a == b {
            b + 3
          } else if a == 1 && b == 2 || a == 2 && b == 3 || a == 3 && b == 1 {
            b + 6
          } else {
            b
          }
        })
        .sum();

  let score2: i32 = games.map(|x| {
          let a: i32 = x.0.clone();
          let b: i32 = x.1.clone();
          if b == 1 {
            if a == 1 {
              3
            } else {
              a - 1
            }
          } else if b == 2 {
            a + 3
          } else {
            a % 3 + 1 + 6
          }
        })
        .sum();

  return format!("{}, {}", score1, score2);
}