fn main() {
    let name = format!("world");
    greeter(name);
}

fn greeter(name: String) {
    println!("Hello, {}!", name);
}
