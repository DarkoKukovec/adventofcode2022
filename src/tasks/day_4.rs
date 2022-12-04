fn get_arr(start: i32, end: i32) -> std::vec::IntoIter<i32> {
    let mut arr = Vec::new();
    for i in start..end+1 {
        arr.push(i);
    }
    return arr.into_iter();
}

pub fn exec(input: &str) -> String {
  let pairs = input
    .split("\n")
    .map(|x|
      x
        .split(',')
        .map(|x|
          x
            .split('-')
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>()
      ).collect::<Vec<Vec<i32>>>()
    )
    .map(|x| -> (usize, usize, usize) {
      // Could be written to be faster (not worth it here), but probably not simpler
      let first = get_arr(x[0][0], x[0][1]);
      let last = get_arr(x[1][0], x[1][1]);
      let overlap_count = first.clone().filter(|x| last.clone().any(|y| y == x.clone())).count();
      return (overlap_count, first.len(), last.len());
    });

  let total_overlap_count = pairs.clone().filter(|x| x.0 == x.1 || x.0 == x.2).count();
  let some_overlap_count = pairs.clone().filter(|x| x.0 > 0).count();

  return format!("{}, {}", total_overlap_count, some_overlap_count);
}