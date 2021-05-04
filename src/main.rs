fn main() {
    println!("Hello, world!");

    let a = "HELLO";
    let b = 1;

    let c : i64 = 2;

    let d : bool = true;

    println!("return: {}", add(b, c));

    let e = String::from("Hello world.");
    println!("{}", e);
}

fn add(x: i64, y: i64) -> i64{
    x + y
}
