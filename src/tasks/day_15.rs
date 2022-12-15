fn count_unique(v: Vec<i32>) -> usize {
    let mut v = v;
    v.sort();
    v.dedup();
    return v.len();
}

fn range_union(r: &Vec<(i32, i32)>) -> Vec<(i32, i32)> {
    let mut r = r.clone();
    r.sort();
    let mut i = 0;
    while i < r.len() - 1 {
        if r[i].1 >= r[i + 1].0 {
            r[i].1 = r[i].1.max(r[i + 1].1);
            r.remove(i + 1);
        } else {
            i += 1;
        }
    }
    return r.to_vec();
}

fn is_in_ranges(x: i32, r: Vec<(i32, i32)>) -> bool {
    for (start, end) in r {
        if x >= start && x <= end {
            return true;
        }
    }
    return false;
}

fn get_empty_ranges(target_row: i32, sensors: &Vec<(i32, i32, i32)>) -> Vec<(i32, i32)> {
    let close_sensors = sensors
        .iter()
        .filter(|s| s.2 - (s.1 - target_row).abs() >= 0)
        .collect::<Vec<&(i32, i32, i32)>>();
    let sensor_dist_to_row = close_sensors
        .iter()
        .map(|s| {
            (
                s.0 - (s.2 - (s.1 - target_row).abs()).max(0),
                s.0 + (s.2 - (s.1 - target_row).abs()).max(0),
            )
        })
        .collect::<Vec<(i32, i32)>>();
    range_union(&sensor_dist_to_row)
}

fn get_nearby(
    target_row: i32,
    sensors: &Vec<(i32, i32, i32)>,
    beacons: &Vec<(i32, i32)>,
    ranges: &Vec<(i32, i32)>,
) -> Vec<i32> {
    let nearby_beacons = beacons
        .iter()
        .filter(|x| x.1 == target_row && is_in_ranges(x.0, ranges.clone()))
        .map(|x| x.0)
        .collect::<Vec<i32>>();
    let nearby_sensors = sensors
        .iter()
        .filter(|x| x.1 == target_row && is_in_ranges(x.0, ranges.clone()))
        .map(|x| x.0)
        .collect::<Vec<i32>>();

    let mut nearby: Vec<i32> = Vec::new();
    nearby.append(&mut nearby_sensors.clone());
    nearby.append(&mut nearby_beacons.clone());
    nearby
}

fn remove_from_range(range: (i32, i32), empty_ranges: Vec<(i32, i32)>) -> Vec<(i32, i32)> {
    let mut ranges = vec![range];
    for empty in empty_ranges {
        let mut new_ranges = Vec::new();
        for (start, end) in ranges {
            if empty.0 > end || empty.1 < start {
                continue;
            }
            if empty.0 > start {
                new_ranges.push((start, empty.0 - 1));
            }
            if empty.1 < end {
                new_ranges.push((empty.1 + 1, end));
            }
        }

        ranges = new_ranges;
    }

    return ranges;
}

fn remove_points_from_ranges(points: Vec<i32>, ranges: Vec<(i32, i32)>) -> Vec<(i32, i32)> {
    let mut ranges = ranges;
    for point in points {
        let mut i = 0;
        while i < ranges.len() {
            if point >= ranges[i].0 && point <= ranges[i].1 {
                if point == ranges[i].0 {
                    ranges[i].0 += 1;
                } else if point == ranges[i].1 {
                    ranges[i].1 -= 1;
                } else {
                    ranges.push((point + 1, ranges[i].1));
                    ranges[i].1 = point - 1;
                }
            }
            i += 1;
        }
    }
    return ranges;
}

fn find_empty_spot(
    target_row: i32,
    max_pos: i32,
    ranges: &Vec<(i32, i32)>,
    sensors: &Vec<(i32, i32, i32)>,
    beacons: &Vec<(i32, i32)>,
) -> Option<i32> {
    let nearby = get_nearby(target_row, sensors, beacons, ranges);
    let full_range = (0, max_pos);
    let empty_ranges =
        remove_points_from_ranges(nearby, remove_from_range(full_range, ranges.clone()));
    if empty_ranges.len() == 0 {
        return None;
    } else if empty_ranges.len() == 1 && empty_ranges[0].0 == empty_ranges[0].1 {
        return Some(empty_ranges[0].0);
    }
    panic!("Multiple empty ranges");
}

pub fn exec(input: &str) -> String {
    let input = input
        .split("\n")
        .map(|x| {
            let line = x.split("=").collect::<Vec<&str>>();
            let s_x = line[1].split(",").next().unwrap().parse::<i32>().unwrap();
            let s_y = line[2].split(":").next().unwrap().parse::<i32>().unwrap();
            let b_x = line[3].split(",").next().unwrap().parse::<i32>().unwrap();
            let b_y = line[4].parse::<i32>().unwrap();
            let distance = (s_x - b_x).abs() + (s_y - b_y).abs();
            return ((s_x, s_y, distance), (b_x, b_y));
        })
        .collect::<Vec<((i32, i32, i32), (i32, i32))>>();

    let sensors: Vec<(i32, i32, i32)> = input.iter().map(|x| x.0).collect();
    let beacons: Vec<(i32, i32)> = input.iter().map(|x| x.1).collect();

    let target_row: i32 = if input.len() == 14 { 10 } else { 2000000 };
    let max_pos: i32 = if input.len() == 14 { 20 } else { 4000000 };

    // Part 1
    let ranges = get_empty_ranges(target_row, &sensors);
    let nearby = get_nearby(target_row, &sensors, &beacons, &ranges);
    let ranges_size: i32 = ranges.iter().map(|x| x.1 - x.0 + 1).sum();
    let empty_in_row: usize = ranges_size as usize - count_unique(nearby);

    // Part 2
    let mut freq: i128 = 0;
    for target_row in 0..max_pos {
        let ranges = get_empty_ranges(target_row, &sensors);
        let empty_spot = find_empty_spot(target_row, max_pos, &ranges, &sensors, &beacons);
        if empty_spot.is_some() {
            freq = (empty_spot.unwrap() as i128) * 4000000 + (target_row as i128);
            break;
        }
    }

    return format!("{:?}, {}", empty_in_row, freq);
}
