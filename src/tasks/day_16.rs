use ndarray::Array3;
use std::collections::HashMap;

// Reddit: https://www.reddit.com/r/adventofcode/comments/zn6k1l/comment/j0gmocd/?utm_source=share&utm_medium=web2x&context=3

pub fn exec(input: &str) -> String {
    let mut valves = input
        .split("\n")
        .map(|x| {
            let line = x.split(" ").collect::<Vec<&str>>();
            let name = line[1].to_string() as String;
            let rate = line[4]
                .split("=")
                .last()
                .unwrap()
                .split(";")
                .next()
                .unwrap()
                .parse::<u16>()
                .unwrap();
            let tunnels = line[9..]
                .to_owned()
                .into_iter()
                .map(|x| x.split(",").next().unwrap())
                .collect::<Vec<&str>>();
            (name, rate, tunnels)
        })
        .collect::<Vec<_>>();
    valves.sort_by(|a, b| b.1.cmp(&a.1));

    let valve_map = valves
        .iter()
        .enumerate()
        .map(|(i, v)| (v.0.to_string(), i))
        .collect::<HashMap<_, _>>();
    let nonzero_valve_count = valves.iter().filter(|v| v.1 > 0).count();
    let total_valve_count = valves.len();
    let mut adj = vec![vec![0usize; 0]; total_valve_count];
    let mut flow = vec![0u16; total_valve_count];
    for v in valves.iter() {
        let i = valve_map[&v.0.to_string()];
        flow[i] = v.1;
        for w in v.2.iter() {
            adj[i].push(valve_map[&w.to_string()]);
        }
    }
    let starting_valve = valve_map["AA"];

    let nonzero_valve_count_mask = 1 << nonzero_valve_count;
    // dynamic programming [time left, current node, bitset of available valves]
    let mut opt = Array3::<u16>::zeros([30, total_valve_count, nonzero_valve_count_mask]);
    for time in 1..30 {
        for valve_index in 0..total_valve_count {
            let valve_index_mask = 1 << valve_index;
            for x in 0..nonzero_valve_count_mask {
                let mut o = opt[(time, valve_index, x)];
                if valve_index_mask & x != 0 && time >= 2 {
                    o = o.max(
                        opt[(time - 1, valve_index, x - valve_index_mask)]
                            + flow[valve_index] * time as u16,
                    );
                }
                for &j in adj[valve_index].iter() {
                    o = o.max(opt[(time - 1, j, x)]);
                }
                opt[(time, valve_index, x)] = o;
            }
        }
    }

    let res1 = opt[(29, starting_valve, nonzero_valve_count_mask - 1)];

    // elephant and human open disjoint sets of valves
    let mut best = 0;
    for x in 0..nonzero_valve_count_mask {
        let y = nonzero_valve_count_mask - 1 - x;
        if (x & y) == 0 {
            best = best.max(opt[(25, starting_valve, x)] + opt[(25, starting_valve, y)]);
        }
    }

    return format!("{}, {}", res1, best);
}
