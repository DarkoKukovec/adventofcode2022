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
    trapped: &mut Vec<(i32, i32, i32)>,
    check_pockets: bool,
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
        } else if check_pockets {
            if !trapped.contains(&point) && check_sides(point, map, empty_vec, trapped, false) == 6
            {
                trapped.push(point);
            }
        }
    }
    count
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
    let mut trapped: Vec<(i32, i32, i32)> = Vec::new();
    for cube in cubes {
        sides_count -= check_sides(cube, &map, &empty_vec, &mut trapped, true);
    }
    super::utils::log(format!("trapped {:?}", trapped).as_str());

    return format!("{}, {}", sides_count, sides_count - trapped.len() * 6);
}
