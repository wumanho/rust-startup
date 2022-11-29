fn main() {
    // if_else_fn();
    for_fn();
}

fn if_else_fn() {
    let condition = true;
    if condition {
        println!("{}", "Haha")
    }
}

fn for_fn(){
    let collection  = [1,2,3];
    // 建议在 for 中使用引用 &，否则所有权会转移
    // 而如果要修改其中的元素，则可以：for i in &mut collection {}
    for i in &collection {
        println!("{}", i)
    }
}
