fn to_dec(input: &str) -> i64 {
    input
        .chars()
        .map(|c| match c {
            '2' => 2,
            '1' => 1,
            '0' => 0,
            '-' => -1,
            '=' => -2,
            _ => panic!("Invalid character"),
        })
        .fold(0, |acc, val| acc * 5 + val)
}

fn to_snafu(num: i64) -> String {
    let mut snafu: Vec<String> = vec![];
    let mut dec_sum = num;

    while dec_sum > 0 {
        let rem = dec_sum % 5;
        snafu.push(match rem {
            2 => "2".to_string(),
            1 => "1".to_string(),
            0 => "0".to_string(),
            3 => "=".to_string(),
            4 => "-".to_string(),
            _ => panic!("Invalid remainder"),
        });
        dec_sum -= rem;
        dec_sum /= 5;
        if rem == 3 || rem == 4 {
            dec_sum += 1;
        }
    }
    snafu.reverse();
    return snafu.join("");
}

pub fn exec(input: &str) -> String {
    let dec_sum = input.lines().map(to_dec).sum::<i64>();
    let snafu = to_snafu(dec_sum);

    return format!("{}, {}", snafu, "Merry Christmas 🎄!");
}
