fn get_visible(lime: &Vec<i32>, size: i32) -> i32 {
    let mut visible = 0;
    for i in 0..lime.len() {
        if lime[i as usize] < size {
            visible += 1;
        } else if lime[i as usize] >= size {
            visible += 1;
            break;
        }
    }
    return visible;
}

pub fn exec(input: &str) -> String {
    let map = input
        .split("\n")
        .map(|x| {
            let items = x
                .chars()
                .map(|x| x.to_string().parse::<i32>().unwrap_or(-1))
                .collect::<Vec<i32>>();
            return items;
        })
        .collect::<Vec<Vec<i32>>>();

    let max_x = map.first().unwrap().len();
    let max_y = map.len();
    let mut visible_from_outside = max_x * 2 + max_y * 2 - 4;

    let mut scenic_score = 0;

    for y in 0..max_y {
        for x in 0..max_x {
            let mut x_before = map[y][0..x].to_vec();
            let x_after = map[y][x + 1..].to_vec();
            let mut y_before = map[0..y].iter().map(|row| row[x]).collect::<Vec<i32>>();
            let y_after = map[y + 1..].iter().map(|row| row[x]).collect::<Vec<i32>>();

            if x < max_x - 1 && y < max_y - 1 && x > 0 && y > 0 {
                if x_before.clone().into_iter().max().unwrap() < map[y][x]
                    || x_after.clone().into_iter().max().unwrap() < map[y][x]
                    || y_before.clone().into_iter().max().unwrap() < map[y][x]
                    || y_after.clone().into_iter().max().unwrap() < map[y][x]
                {
                    visible_from_outside += 1;
                }
            }

            x_before.reverse();
            y_before.reverse();

            let x_before_count = get_visible(&x_before, map[y][x]);
            let x_after_count = get_visible(&x_after, map[y][x]);
            let y_before_count = get_visible(&y_before, map[y][x]);
            let y_after_count = get_visible(&y_after, map[y][x]);

            let current_scenic_score =
                x_before_count * x_after_count * y_before_count * y_after_count;

            if current_scenic_score > scenic_score {
                scenic_score = current_scenic_score;
            }
        }
    }

    return format!("{}, {}", visible_from_outside, scenic_score);
}
