fn calculate_production(
    blueprint: &(i32, i32, i32, i32, i32, i32, i32),
    (
        ore_prod,
        clay_prod,
        obsidian_prod,
        geode_prod,
        ore_count,
        clay_count,
        obsidian_count,
        geode_count,
        time,
    ): &(i32, i32, i32, i32, i32, i32, i32, i32, i32),
) -> i32 {
    if time >= &24 {
        return *geode_count;
    }

    super::utils::log(time.to_string().as_str());

    let (_id, ore_ore, clay_ore, obsidian_ore, obsidian_clay, geode_ore, geode_obsidian) =
        blueprint;

    let ore_count = ore_count + ore_prod;
    let clay_count = clay_count + clay_prod;
    let obsidian_count = obsidian_count + obsidian_prod;
    let geode_count = geode_count + geode_prod;
    let mut geode_max: Vec<i32> = Vec::new();

    if geode_obsidian < &obsidian_count && geode_ore < &ore_count {
        geode_max.push(calculate_production(
            blueprint,
            &(
                *ore_prod,
                *clay_prod,
                *obsidian_prod,
                geode_prod + 1,
                ore_count - geode_ore,
                clay_count,
                obsidian_count - geode_obsidian,
                geode_count,
                time + 1,
            ),
        ));
    }
    if obsidian_clay < &clay_count && obsidian_ore < &ore_count {
        geode_max.push(calculate_production(
            blueprint,
            &(
                *ore_prod,
                *clay_prod,
                obsidian_prod + 1,
                *geode_prod,
                ore_count - obsidian_ore,
                clay_count - obsidian_clay,
                obsidian_count,
                geode_count,
                time + 1,
            ),
        ));
    }
    if clay_ore < &ore_count {
        geode_max.push(calculate_production(
            blueprint,
            &(
                *ore_prod,
                clay_prod + 1,
                *obsidian_prod,
                *geode_prod,
                ore_count - clay_ore,
                clay_count,
                obsidian_count,
                geode_count,
                time + 1,
            ),
        ));
    }
    if ore_ore < &ore_count {
        geode_max.push(calculate_production(
            blueprint,
            &(
                ore_prod + 1,
                *clay_prod,
                *obsidian_prod,
                *geode_prod,
                ore_count - ore_ore,
                clay_count,
                obsidian_count,
                geode_count,
                time + 1,
            ),
        ));
    }
    geode_max.push(calculate_production(
        blueprint,
        &(
            *ore_prod,
            *clay_prod,
            *obsidian_prod,
            *geode_prod,
            ore_count,
            clay_count,
            obsidian_count,
            geode_count,
            time + 1,
        ),
    ));

    *geode_max.iter().max().unwrap()
}

pub fn exec(input: &str) -> String {
    let blueprints = input
        .lines()
        .map(|x| {
            let id = x
                .split(":")
                .next()
                .unwrap()
                .split(" ")
                .last()
                .unwrap()
                .parse::<i32>()
                .unwrap();
            let mut parts = x.split(". ");
            let ore = parts.next().unwrap().split(" ").collect::<Vec<_>>();
            let ore_ore = ore[6].parse::<i32>().unwrap();
            let clay = parts.next().unwrap().split(" ").collect::<Vec<_>>();
            let clay_ore = clay[4].parse::<i32>().unwrap();
            let obsidian = parts.next().unwrap().split(" ").collect::<Vec<_>>();
            let obsidian_ore = obsidian[4].parse::<i32>().unwrap();
            let obsidian_clay = obsidian[7].parse::<i32>().unwrap();
            let geode = parts.next().unwrap().split(" ").collect::<Vec<_>>();
            let geode_ore = geode[4].parse::<i32>().unwrap();
            let geode_obsidian = geode[7].parse::<i32>().unwrap();

            (
                id,
                ore_ore,
                clay_ore,
                obsidian_ore,
                obsidian_clay,
                geode_ore,
                geode_obsidian,
            )
        })
        .collect::<Vec<_>>();

    let mut sum = 0;

    for blueprint in &blueprints {
        let res = calculate_production(blueprint, &(1, 0, 0, 0, 0, 0, 0, 0, 0));
        super::utils::log(format!("{:?}", res).as_str());
        sum += blueprint.0 * res;
    }

    super::utils::log(format!("{:?}", blueprints).as_str());

    return format!("{}, {}", sum, "world");
}
