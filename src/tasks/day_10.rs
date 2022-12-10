fn get_power(cmds: &Vec<i32>, count: i32) -> i32 {
    (1 + cmds[0..(count - 1) as usize].into_iter().sum::<i32>()) * count
}

pub fn exec(input: &str) -> String {
    let cmds_init = input.split("\n").map(|x| {
        let mut data = x.split(" ");
        (
            match data.next().unwrap() {
                "addx" => 1,
                _ => 0,
            },
            data.next().unwrap_or("0").parse::<i32>().unwrap(),
        )
    });

    let mut cmds = Vec::<i32>::new();
    for cmd in cmds_init {
        if cmd.0 == 1 {
            cmds.push(0);
        }
        cmds.push(cmd.1);
    }

    let mut pos = 20;
    let mut sum: i32 = 0;

    while pos < cmds.len() as i32 {
        sum += get_power(&cmds, pos);
        pos += 40;
    }

    let mut screen = String::new();
    let mut value: i32 = 0;
    for x in 0..6 {
        for y in -1..39 {
            let pos = x * 40 + y;
            if pos >= 0 {
                value += cmds[pos as usize];
            }
            if (value - y).abs() <= 1 {
                screen.push_str("█");
            } else {
                screen.push_str(" ");
            }
        }
        screen.push_str("\n");
    }

    return format!("{}\n\n{}", sum, screen);
}
