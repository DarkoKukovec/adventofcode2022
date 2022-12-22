use std::collections::HashMap;

#[derive(Debug)]
enum Instruction {
    Move(i32),
    Rotate(i32),
}

const DIRS: [(i8, i8); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

fn get_first_x(y: i32, map: &Vec<Vec<char>>) -> i32 {
    // super::utils::log(format!("First x: {:?}", y).as_str());
    return map[y as usize]
        .iter()
        .enumerate()
        .find(|&(_, x)| x == &'.' || x == &'#')
        .unwrap()
        .0 as i32;
}

fn get_last_x(y: i32, map: &Vec<Vec<char>>) -> i32 {
    // super::utils::log(format!("Last x: {:?}", y).as_str());
    return map[y as usize]
        .iter()
        .enumerate()
        .rev()
        .find(|&(_, x)| x == &'.' || x == &'#')
        .unwrap()
        .0 as i32;
}

fn get_first_y(x: i32, map: &Vec<Vec<char>>) -> i32 {
    // super::utils::log(format!("First y: {:?}", x).as_str());
    return map
        .iter()
        .enumerate()
        .find(|&(_, row)| {
            row.len() > x as usize && (row[x as usize] == '.' || row[x as usize] == '#')
        })
        .unwrap()
        .0 as i32;
}

fn get_last_y(x: i32, map: &Vec<Vec<char>>) -> i32 {
    // super::utils::log(format!("Last y: {:?}", x).as_str());
    return map
        .iter()
        .enumerate()
        .rev()
        .find(|&(_, row)| {
            row.len() > x as usize && (row[x as usize] == '.' || row[x as usize] == '#')
        })
        .unwrap()
        .0 as i32;
}

pub fn exec(input: &str) -> String {
    let mut input = input.split("\n\n");
    let map = input
        .next()
        .unwrap()
        .lines()
        .map(|x| x.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let instr = input.next().unwrap().chars();
    // super::utils::log(format!("{:?}", instr).as_str());
    let mut m = 0;
    let mut instructions: Vec<Instruction> = Vec::new();
    for i in instr {
        if i == 'L' || i == 'R' {
            instructions.push(Instruction::Move(m));
            instructions.push(Instruction::Rotate(if i == 'L' { -1 } else { 1 }));
            m = 0;
        } else {
            m *= 10;
            m += i.to_digit(10).unwrap() as i32;
        }
    }
    instructions.push(Instruction::Move(m));

    // super::utils::log(format!("{:?}", instructions).as_str());

    let mut x: i32 = get_first_x(0, &map);
    let mut y: i32 = 0;
    // super::utils::log(format!("{:?} {}", x, y).as_str());
    // super::utils::log(format!("{} {}",  map.len(), map[0].len()).as_str());
    // super::utils::log(format!("{:?}", get_first_y(12, &map)).as_str());

    let mut dir = (1, 0);
    let mut dir_index = 0;
    for i in instructions {
        // super::utils::log(format!("{:?} {:?}", i, dir).as_str());
        match i {
            Instruction::Move(m) => {
                for _i in 0..m {
                    let mut tmp_x = x + dir.0 as i32;
                    let mut tmp_y = y + dir.1 as i32;
                    // super::utils::log(format!("Next: {:?} {}", tmp_x, tmp_y).as_str());

                    if tmp_y < 0 {
                        tmp_y = get_last_y(tmp_x, &map);
                        // super::utils::log(format!(": {:?} {}", tmp_x, tmp_y).as_str());
                    } else if (tmp_y as usize) >= map.len() {
                        tmp_y = get_first_y(tmp_x, &map);
                    }
                    if tmp_x < 0 {
                        tmp_x = get_last_x(tmp_y, &map);
                    } else if (tmp_x as usize) >= map[tmp_y as usize].len() {
                        tmp_x = get_first_x(tmp_y, &map);
                    }
                    if map[tmp_y as usize][tmp_x as usize] == ' ' {
                        if dir.0 == 1 {
                            tmp_x = get_first_x(tmp_y, &map);
                        } else if dir.0 == -1 {
                            tmp_x = get_last_x(tmp_y, &map);
                        } else if dir.1 == 1 {
                            tmp_y = get_first_y(tmp_x, &map);
                        } else if dir.1 == -1 {
                            tmp_y = get_last_y(tmp_x, &map);
                            // super::utils::log(format!(": {:?} {}", tmp_x, tmp_y).as_str());
                        }
                    }

                    if map[tmp_y as usize][tmp_x as usize] == '#' {
                        break;
                    }
                    x = tmp_x;
                    y = tmp_y;
                }
                // super::utils::log(format!("{} {} {:?}", x, y, dir).as_str());
            }
            Instruction::Rotate(d) => {
                dir_index += d;
                dir_index = (dir_index + 4) % 4;
                dir = DIRS[dir_index as usize];
            }
        }
    }

    let mut values: HashMap<(i8, i8), i32> = HashMap::new();
    values.insert((1, 0), 0);
    values.insert((0, 1), 1);
    values.insert((-1, 0), 2);
    values.insert((0, -1), 3);

    super::utils::log(format!("{} {} {:?}", x, y, dir).as_str());

    let res = (y + 1) * 1000 + (x + 1) * 4 + values[&dir];

    return format!("{}, {}", res, "world");
}

// < 137110
