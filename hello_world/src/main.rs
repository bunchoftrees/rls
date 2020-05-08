use std::io::{self, Write};

fn main() {
    greeter("World".to_string());
    greeter(get_name());
}

fn greeter(name: String) {
    println!("Hello, {}!", name);
    io::stdout().flush().unwrap();
}

fn get_name() -> String {
    print!("Please input your name:  ");
    io::stdout().flush().unwrap();
    let mut name = String::new();
    io::stdin()
            .read_line(&mut name)
            .expect("Failed to read line");
    let name = name.trim();
    return name.to_string();
}
