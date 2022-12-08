fn main() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("{:?}", six);
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    return match x {
        None => None,
        Some(i) => Some(i + 1),
    };
}
