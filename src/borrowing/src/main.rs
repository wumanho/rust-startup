fn main() {
    // inner();
    // unchangeable();

    // let s1 = String::from("hello");
    // change(s1); 报错，因为s1是不可变的

    let mut s1 = String::from("hello");
    change(&mut s1); // &mut s1 叫做可变引用，注意同一作用域只允许一个
}

fn inner() {
    let x = 5;
    let y = &x; // y 实际上就是指向 x 的值的指针
    assert_eq!(5, x);
    assert_eq!(5, *y); // * 解引用
}

fn unchangeable() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len)
}

fn calculate_length(s: &String) -> usize {
    return s.len();
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
