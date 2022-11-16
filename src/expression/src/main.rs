fn main() {
    //add_with_extra(3, 4);
    assert_eq!(ret_unit_type(), ());
}

/**
 * 当最后一行为表达式时，不需要写 return
 */
fn add_with_extra(x: i32, y: i32) -> i32 {
    let x = x + 1; //语句
    let y = y + 5; // 语句
    x + y //表达式，表达式作为返回值不能以;结尾
}

// 这里没有返回任何值，会隐式返回 ()
fn ret_unit_type() {
    let x = 1;
    let y = if x % 2 == 1 { "odd" } else { "even" };
}
