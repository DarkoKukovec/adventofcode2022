use std::collections::HashMap;

pub fn exec(input: &str) -> String {
  let state = input
    .split("\n\n")
    .collect::<Vec<&str>>()
    .first()
    .unwrap()
    .split("\n")
    .collect::<Vec<&str>>();

  let stack_names = state
    .clone()
    .last()
    .unwrap()
    .chars()
    .filter(|x| x != &' ')
    .clone();
  
  let mut s = state.clone();
  s.reverse();
  let stack = s.iter().skip(1);

  let mut stacks: HashMap<String, Vec<&str>> = HashMap::new();
  let names = stack_names.clone();
  for stack_name in names {
    let index = state
      .clone()
      .last()
      .unwrap()
      .find(stack_name)
      .unwrap();

    let items: Vec<&str> = stack
      .clone()
      .map(|x| {
        let box_name: &str = x[index..index+1].trim();
        box_name
      })
      .filter(|x| x != &"")
      .collect();
    stacks.insert(stack_name.to_string(), items);
  }

  let moves = input
    .split("\n\n")
    .collect::<Vec<&str>>()
    .last()
    .unwrap()
    .split("\n")
    .map(|x| x.split(" ").map(|y| y.parse::<i32>().unwrap_or(0)))
    .map(|x| (
      x.clone().nth(1).unwrap(),
      x.clone().nth(3).unwrap().to_string(),
      x.clone().nth(5).unwrap().to_string()
    ));

  // First part
  let mut stack_single = stacks.clone();
  for next_move in moves.clone() {
    for i in 0..next_move.0 {
      let item = stack_single.get_mut(&next_move.1).unwrap().pop().unwrap();
      stack_single.get_mut(&next_move.2).unwrap().push(item);
    }
  }
  let result_single = stack_names
    .clone()
    .map(|x| stack_single.get(&x.to_string()).unwrap().last().unwrap())
    .fold(String::new(), |acc, x| acc + x);

  // Second part
  let mut stack_multi = stacks.clone();
  for next_move in moves.clone() {
    let mut items: Vec<&str> = Vec::new();
    for i in 0..next_move.0 {
      let item = stack_multi.get_mut(&next_move.1).unwrap().pop().unwrap();
      items.push(item);
    }
    items.reverse();
    for item in items {
      stack_multi.get_mut(&next_move.2).unwrap().push(item);
    }
  }
  let result_multi = stack_names
    .map(|x| stack_multi.get(&x.to_string()).unwrap().last().unwrap())
    .fold(String::new(), |acc, x| acc + x);

  return format!("{}, {}", result_single, result_multi);
}