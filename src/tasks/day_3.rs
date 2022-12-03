fn char_to_val(ch: char) -> i32 {
    let b = ch as i32 - 64;
    if b > 32 {
        b - 32
    } else {
        b + 26
    }
}

fn find_score(input: &str, others: Vec<&str>) -> i32 {
  let ch_index = input.find(|c| others.iter().all(|x| x.contains(c))).unwrap();
  let ch = input.chars().nth(ch_index).unwrap();
  char_to_val(ch)
}

pub fn exec(input: &str) -> String {  
  let rucksacks = input.split("\n");

  let weighted: i32 = rucksacks.clone().map(|rucksack| {
    let half = rucksack.len() / 2;
    find_score(&rucksack[0..half], [&rucksack[half..]].to_vec())
  }).sum();

  let group_count = rucksacks.clone().count() / 3;
  let mut badge_sum = 0;
  for i in 0..group_count {
    let group = rucksacks.clone().skip(i * 3).take(3);
    badge_sum += find_score(group.clone().next().unwrap(), group.collect());
  }

  format!("{}, {}", weighted, badge_sum)
}