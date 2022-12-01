pub fn exec(input: &str) -> String {
    let mut calories: Vec<i32> = input
        .split("\n\n")
        .map(|x| x.split("\n").map(super::utils::str_to_i32))
        .map(|x| x.sum())
        .collect();
    
    calories.sort();
    calories.reverse();

    return format!("{}, {}", calories[0], calories[0..3].iter().sum::<i32>());
}