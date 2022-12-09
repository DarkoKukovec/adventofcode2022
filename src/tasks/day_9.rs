use std::collections::HashMap;

pub fn exec(input: &str) -> String {
    let moves = input.split("\n").map(|x| {
        let mut dir = x.split(" ");
        (
            dir.next().unwrap().chars().next().unwrap(),
            dir.next().unwrap().parse::<i32>().unwrap(),
        )
    });
    let mut positions_1: HashMap<(i32, i32), bool> = HashMap::new();
    let mut positions_9: HashMap<(i32, i32), bool> = HashMap::new();
    let mut points: [(i32, i32); 10] = [
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
    ];

    for next_move in moves {
        for _ in 0..next_move.1 {
            match next_move.0 {
                'U' => points[0].1 += 1,
                'D' => points[0].1 -= 1,
                'L' => points[0].0 -= 1,
                'R' => points[0].0 += 1,
                _ => {}
            }

            for i in 1..10 {
                let diff_x: i32 = points[i - 1].0 - points[i].0;
                let diff_y: i32 = points[i - 1].1 - points[i].1;

                if diff_x.pow(2) + diff_y.pow(2) > 2 {
                    points[i].0 += diff_x.clamp(-1, 1);
                    points[i].1 += diff_y.clamp(-1, 1);
                }
            }

            positions_1.insert(points[1], true);
            positions_9.insert(points[9], true);
        }
    }

    let position_1_count = positions_1.len();
    let position_9_count = positions_9.len();

    return format!("{}, {}", position_1_count, position_9_count);
}
