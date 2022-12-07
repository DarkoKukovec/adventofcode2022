use std::collections::HashMap;

const SIZE_LIMIT: i32 = 100000;
const TOTAL_SPACE: i32 = 70000000;
const REQUIRED_SPACE: i32 = 30000000;

pub fn exec(input: &str) -> String {
    let output = input.split("\n").map(|x| -> (i32, &str) {
        let mut parts = x.split(" ").into_iter();
        let first = parts.next().unwrap();
        let second = parts.next().unwrap();
        let third = parts.next().unwrap_or("");
        if first == "$" {
            return (0, third);
        } else if first == "dir" {
            return (1, second);
        }
        (2, first)
    });

    let mut current_folder = Vec::<&str>::new();
    let mut folders = HashMap::<String, i32>::new();
    let mut add_sizes = Vec::<(String, String)>::new();
    for output in output {
        let (type_, value) = output;
        if type_ == 0 {
            if value == "" {
                continue;
            } else if value == ".." {
                current_folder.pop();
            } else {
                current_folder.push(value);
            }
        } else if type_ == 1 {
            add_sizes.push((current_folder.join("/"), value.to_string()));
        } else {
            let folder = current_folder.join("/");
            let size = folders.get(folder.as_str()).unwrap_or(&0) + value.parse::<i32>().unwrap();
            folders.insert(folder, size);
        }
    }
    add_sizes.reverse();
    for (parent, current) in add_sizes {
        let size = folders.get(parent.as_str()).unwrap_or(&0)
            + folders
                .get([parent.clone(), current.clone()].join("/").as_str())
                .unwrap_or(&0);
        folders.insert(parent, size);
    }
    let to_delete_small = folders
        .iter()
        .filter(|x| x.1 < &SIZE_LIMIT)
        .fold(0, |acc, x| acc + x.1);

    let free_space = TOTAL_SPACE - folders.get("/").unwrap_or(&0);
    let required_space = REQUIRED_SPACE - free_space;

    let to_delete_one = folders
        .iter()
        .filter(|x| x.1 > &required_space)
        .min_by_key(|x| x.1)
        .unwrap()
        .1
        .clone();

    return format!("{}, {}", to_delete_small, to_delete_one);
}
