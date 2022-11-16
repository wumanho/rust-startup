fn main() {
    ownership();
}

fn ownership() {
    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("{}", s)
}
