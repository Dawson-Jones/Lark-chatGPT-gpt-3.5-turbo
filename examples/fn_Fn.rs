fn add(x: i32, y: i32) -> i32 {
    x + y
}

fn main() {
    let f: fn(i32, i32) -> i32 = add;
    let res = f(1, 2);

    println!("res: {res}");

    let x = 1;
    let f  = | y |  x + y;

    let res = f(2);
}