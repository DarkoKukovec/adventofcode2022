const MOVE_PRIORITY: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

fn get_empty(positions: &Vec<(i32, i32)>) -> i32 {
    let min_x = positions.iter().map(|i| i.0).min().unwrap();
    let max_x = positions.iter().map(|i| i.0).max().unwrap();
    let min_y = positions.iter().map(|i| i.1).min().unwrap();
    let max_y = positions.iter().map(|i| i.1).max().unwrap();

    (max_x - min_x + 1).abs() * (max_y - min_y + 1).abs() - positions.len() as i32
}

fn do_move(positions: &Vec<(i32, i32)>, move_index: usize) -> (bool, Vec<(i32, i32)>) {
    let priorities: [(i32, i32); 4] = [
        MOVE_PRIORITY[move_index % 4],
        MOVE_PRIORITY[(move_index + 1) % 4],
        MOVE_PRIORITY[(move_index + 2) % 4],
        MOVE_PRIORITY[(move_index + 3) % 4],
    ];

    let mut possible_moves: Vec<Option<(i32, i32)>> = vec![];
    let mut doubles: Vec<(i32, i32)> = vec![];
    for position in positions.iter() {
        if !positions.contains(&(position.0 + 1, position.1))
            && !positions.contains(&(position.0 - 1, position.1))
            && !positions.contains(&(position.0, position.1 + 1))
            && !positions.contains(&(position.0, position.1 - 1))
            && !positions.contains(&(position.0 + 1, position.1 + 1))
            && !positions.contains(&(position.0 + 1, position.1 - 1))
            && !positions.contains(&(position.0 - 1, position.1 + 1))
            && !positions.contains(&(position.0 - 1, position.1 - 1))
        {
            possible_moves.push(None);
            continue;
        }
        let mut can_move = false;
        for priority in priorities.iter() {
            let new_position = (position.0 + priority.0, position.1 + priority.1);
            let (left, right) = if priority.0 == 0 {
                (
                    (position.0 + priority.0 - 1, position.1 + priority.1),
                    (position.0 + priority.0 + 1, position.1 + priority.1),
                )
            } else {
                (
                    (position.0 + priority.0, position.1 + priority.1 - 1),
                    (position.0 + priority.0, position.1 + priority.1 + 1),
                )
            };
            // super::utils::log(format!("{:?} {:?} {:?} {:?} {:?}", position, priority, new_position, left, right).as_str());
            if !positions.contains(&new_position)
                && !positions.contains(&left)
                && !positions.contains(&right)
            {
                can_move = true;
                if possible_moves.contains(&Some(new_position)) {
                    doubles.push(new_position);
                }
                possible_moves.push(Some(new_position));
                break;
            }
        }
        if !can_move {
            possible_moves.push(None);
        }
    }

    let mut moves_count = 0;

    let new_positions = possible_moves
        .iter()
        .enumerate()
        .map(|(i, pos)| {
            if pos.is_some() && !doubles.contains(&pos.unwrap()) {
                moves_count += 1;
                pos.unwrap()
            } else {
                positions[i]
            }
        })
        .collect::<Vec<_>>();

    return (moves_count > 0, new_positions);
}

pub fn exec(input: &str) -> String {
    let mut positions: Vec<(i32, i32)> = input
        .lines()
        .enumerate()
        .map(|(x, line)| {
            line.chars()
                .enumerate()
                .map(|(y, c)| {
                    if c == '#' {
                        Some((x as i32, y as i32))
                    } else {
                        None
                    }
                })
                .filter(|i| i.is_some())
                .collect::<Vec<_>>()
        })
        .flatten()
        .map(|i| i.unwrap())
        .collect::<Vec<_>>();

    let mut move_index = 0;
    let mut empty: i32 = 0;
    loop {
        let (has_moves, new_positions) = do_move(&positions, move_index);
        positions = new_positions;

        if move_index == 9 {
            empty = get_empty(&positions);
        }

        if !has_moves {
            break;
        }

        move_index += 1;
    }

    return format!("{}, {}", empty, move_index + 1);
}
