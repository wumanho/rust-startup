fn main() {
    // ownership();
    let s = String::from("hello");
    // 这个函数会借用 s 的值
    takes_ownership(s);
    // 会报错
    println!("{}", s)
}

fn ownership() {
    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("{}", s)
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string)
}
