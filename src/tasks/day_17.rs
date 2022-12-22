const EMPTY_ROWS: usize = 3;

fn try_move(block: &mut Vec<(i128, i128)>, max_height: &[usize; 7], dir: (i128, i128)) -> bool {
    let new_block = block.clone();
    let new_block = new_block.iter().map(|(x, y)| (x + dir.0, y + dir.1));
    let valid_positions = new_block
        .clone()
        .filter(|(x, y)| {
            if x < &0 || y < &0 || x > &6 {
                return false;
            }
            (max_height[*x as usize] as i128) < *y
        })
        .collect::<Vec<_>>();

    super::utils::log(format!("Valid: {:?}, All: {:?}", valid_positions, block).as_str());
    if valid_positions.len() == block.len() {
        block.clear();
        new_block.for_each(|x| block.push(x));
        return true;
    }
    false
}

pub fn exec(input: &str) -> String {
    let directions = input.chars().collect::<Vec<char>>();
    super::utils::log(format!("directions {:?}", directions).as_str());
    let mut max_height: [usize; 7] = [0, 0, 0, 0, 0, 0, 0];

    let objects: [Vec<(i128, i128)>; 5] = [
        vec![(2, 0), (3, 0), (4, 0), (5, 0)],
        vec![(2, 1), (3, 0), (3, 1), (3, 2), (4, 1)],
        vec![(2, 0), (3, 0), (4, 0), (4, 1), (4, 2)],
        vec![(2, 0), (2, 1), (2, 2), (2, 3)],
        vec![(2, 0), (3, 0), (2, 1), (3, 1)],
    ];
    // super::utils::log(format!("objects {:?}", objects).as_str());

    let mut block_count: usize = 0;
    let mut move_count: usize = 0;

    while block_count < 2 {
        // super::utils::log(format!("index {:?}", block_count % objects.len()).as_str());
        let block_item = objects[block_count % objects.len()].clone();
        // super::utils::log(format!("block_item {:?}", block_item).as_str());
        let mut block = block_item
            .iter()
            .map(|x| {
                (
                    x.0,
                    x.1 + *max_height.iter().max().unwrap() as i128 + EMPTY_ROWS as i128,
                )
            })
            .collect::<Vec<_>>();

        super::utils::log(format!("Block {:?}", block).as_str());
        loop {
            let dir = directions.get(move_count % directions.len()).unwrap();
            super::utils::log(format!("Side {:?}", dir).as_str());
            let diff = if *dir == '<' { -1 } else { 1 };
            move_count += 1;
            try_move(&mut block, &max_height, (diff, 0));

            super::utils::log(format!("Down").as_str());
            let valid = try_move(&mut block, &max_height, (0, -1));
            if !valid {
                super::utils::log(format!("Landed {:?}", block).as_str());
                for (x, y) in block {
                    max_height[x as usize] = max_height[x as usize].max(y as usize);
                }
                //     while hall.get(item.1 as usize).is_none() {
                //         hall.push(EMPTY_ROW.clone());
                //     }
                //     let row = hall.get_mut(item.1 as usize).unwrap();
                //     row[item.0 as usize] = true;
                // }
                // let mut display = hall
                //     .iter()
                //     .map(|x| x.map(|y| if y { "#" } else { "." }).join(""))
                //     .collect::<Vec<_>>();
                // display.reverse();
                super::utils::log(format!("{:?}", max_height).as_str());
                break;
            }
        }

        block_count += 1;
    }

    return format!("{}, {}", max_height.iter().max().unwrap(), "world");
}
