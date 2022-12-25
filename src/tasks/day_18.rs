use std::collections::HashMap;

const MOVE_MAP: [(i32, i32, i32); 6] = [
    (1, 0, 0),
    (-1, 0, 0),
    (0, 1, 0),
    (0, -1, 0),
    (0, 0, 1),
    (0, 0, -1),
];

fn check_sides(
    (x, y, z): (i32, i32, i32),
    map: &HashMap<(i32, i32), Vec<i32>>,
    empty_vec: &Vec<i32>,
    border: &mut HashMap<(i32, i32, i32), usize>,
) -> usize {
    let mut count: usize = 0;
    for diff in MOVE_MAP {
        let point = (x + diff.0, y + diff.1, z + diff.2);
        if map
            .get(&(point.0, point.1))
            .unwrap_or(&empty_vec)
            .contains(&point.2)
        {
            count += 1;
        } else {
            let num = border.get(&point).unwrap_or(&0);
            border.insert(point, num + 1);
        }
    }
    count
}

fn is_neighbor((x, y, z): (i32, i32, i32), list: &Vec<(i32, i32, i32)>) -> bool {
    for diff in MOVE_MAP {
        let point = (x + diff.0, y + diff.1, z + diff.2);
        if list.contains(&point) {
            return true;
        }
    }
    return false;
}

pub fn exec(input: &str) -> String {
    let mut map: HashMap<(i32, i32), Vec<i32>> = HashMap::new();
    let cubes = input
        .lines()
        .map(|x| {
            let mut c = x.split(",");
            let x = c.next().unwrap().parse::<i32>().unwrap();
            let y = c.next().unwrap().parse::<i32>().unwrap();
            let z = c.next().unwrap().parse::<i32>().unwrap();
            let list = map.get_mut(&(x, y));
            if list.is_none() {
                map.insert((x, y), vec![z]);
            } else {
                list.unwrap().push(z);
            }
            (x, y, z)
        })
        .collect::<Vec<_>>();

    super::utils::log(format!("Cubes {:?}", cubes).as_str());
    super::utils::log(format!("Map {:?}", map).as_str());

    let mut sides_count: usize = cubes.len() * 6;
    let empty_vec: Vec<i32> = Vec::new();
    let mut border: HashMap<(i32, i32, i32), usize> = HashMap::new();
    for cube in cubes.iter() {
        sides_count -= check_sides(*cube, &map, &empty_vec, &mut border);
    }

    super::utils::log(format!("trapped {:?}", border.len()).as_str());
    let mut trapped: Vec<(i32, i32, i32)> = Vec::new();
    let mut free: Vec<(i32, i32, i32)> = Vec::new();
    for (point, count) in border.iter() {
        if count == &6 {
            trapped.push(*point);
        } else {
            if is_neighbor(*point, &trapped) {
                trapped.push(*point);
            } else if is_neighbor(*point, &free) {
                free.push(*point);
            }
            // free.push(point);
        }
    }

    super::utils::log(
        format!(
            "min {:?} {:?} {:?}, max {:?} {:?} {:?}",
            cubes.iter().min_by(|a, b| a.0.cmp(&b.0)).unwrap().0,
            cubes.iter().min_by(|a, b| a.1.cmp(&b.1)).unwrap().1,
            cubes.iter().min_by(|a, b| a.2.cmp(&b.2)).unwrap().2,
            cubes.iter().max_by(|a, b| a.0.cmp(&b.0)).unwrap().0,
            cubes.iter().max_by(|a, b| a.1.cmp(&b.1)).unwrap().1,
            cubes.iter().max_by(|a, b| a.2.cmp(&b.2)).unwrap().2,
        )
        .as_str(),
    );

    let trapped_area = trapped
        .iter()
        .map(|p| border.get(&p).unwrap_or(&0))
        .sum::<usize>();
    return format!("{}, {}", sides_count, sides_count - trapped_area);
}
