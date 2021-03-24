fn main() {
    println!("Hello, world!");

    another_function();
    five_function(5);

    let x = five();

    println!(
        "{}",x
    );
}

fn another_function() {
    println!("Another function.");
}

fn five_function(x:i32) {
    println!("{}",x);
}

fn five() -> i32 {
    5
}