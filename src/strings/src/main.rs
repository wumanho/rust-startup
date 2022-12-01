fn main() {
    let mut my_name = String::from("Pascal");
    greet(&mut my_name);
    // println!("{}", my_name)

    // 1.切片
    // create_slice();
}

fn greet(name: &mut String) {
    name.push_str(" rust");
    println!("Hello,{}!", name)
}

// fn create_slice() {
//     let s = String::from("hello world");
//     let hello = &s[0..5];
//     let world = &s[6..11];
//     // 索引从0开始
//     let from_zero = &s[..2];
//     // 包含最后一个字节
//     let len = &s[2..len()];
// }
