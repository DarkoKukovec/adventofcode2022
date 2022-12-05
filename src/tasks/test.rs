pub fn exec(name: &str) -> String {
    super::utils::log(&format!("Hello from console, {name}!"));
    return format!("Hello, {name}!");
}
