fn main() {
    println!("Hello, world!");

    let name = "Brandon";
    let mut count = 0;
    for i in 1..=5 {
        count = i;
        println!("Hello, {name}! (count: {count})");
    }

    match count {
        count if count % 2 == 0 => println!("count is even!"),
        _ => println!("count is odd!"),
    }
}
