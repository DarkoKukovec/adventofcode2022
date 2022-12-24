fn exec_trip(
    start: (i32, i32),
    end: (i32, i32),
    winds_: &mut Vec<(i32, i32, char)>,
    max_x: i32,
    max_y: i32,
) -> usize {
    let mut paths: Vec<Vec<(i32, i32)>> = vec![vec![start]];
    let mut winds = winds_.clone();
    loop {
        // Update winds
        winds = winds
            .iter()
            .map(|(x, y, c)| {
                let (x, y) = match c {
                    '>' => (x + 1, *y),
                    '<' => (x - 1, *y),
                    '^' => (*x, y - 1),
                    'v' => (*x, y + 1),
                    _ => panic!("Invalid wind direction"),
                };
                let x = if x > max_x {
                    1
                } else if x < 1 {
                    max_x
                } else {
                    x
                };
                let y = if y > max_y {
                    1
                } else if y < 1 {
                    max_y
                } else {
                    y
                };
                (x, y, *c)
            })
            .collect::<Vec<_>>();

        // Check paths
        let mut visited: Vec<(i32, i32)> = Vec::new();
        let mut new_paths: Vec<Vec<(i32, i32)>> = Vec::new();
        for path in paths {
            let last = path.last().unwrap();
            let next_options = [
                *last, // Stay
                (last.0 + 1, last.1),
                (last.0 - 1, last.1),
                (last.0, last.1 + 1),
                (last.0, last.1 - 1),
            ];

            for next in next_options.iter() {
                if winds.iter().any(|(x, y, _)| x == &next.0 && y == &next.1) {
                    continue;
                }

                if visited.contains(next) {
                    continue;
                }

                if next == &end {
                    *winds_ = winds;
                    // super::utils::log(format!("{:?} {:?}", path, next).as_str());
                    return path.len();
                }

                if next == &start
                    || (next.0 >= 1 && next.0 <= max_x && next.1 >= 1 && next.1 <= max_y)
                {
                    let mut new_path = path.clone();
                    visited.push(*next);
                    new_path.push(*next);
                    new_paths.push(new_path);
                }
            }
        }
        paths = new_paths;

        if paths.len() == 0 {
            panic!("No paths found");
        }
    }
}

pub fn exec(input: &str) -> String {
    let max_x = input.lines().next().unwrap().len() as i32 - 2;
    let max_y = input.lines().count() as i32 - 2;
    let start = (1, 0);
    let end = (max_x, max_y + 1);
    let mut winds: Vec<(i32, i32, char)> = Vec::new();
    input.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, c)| {
            if c == '>' || c == '<' || c == '^' || c == 'v' {
                winds.push((x as i32, y as i32, c));
            }
        });
    });

    let part1 = exec_trip(start, end, &mut winds, max_x, max_y);
    let to_start = exec_trip(end, start, &mut winds, max_x, max_y);
    let part2 = exec_trip(start, end, &mut winds, max_x, max_y);

    return format!("{}, {}", part1, part1 + to_start + part2);
}
