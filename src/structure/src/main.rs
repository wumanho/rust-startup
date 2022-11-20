fn main() {
    // 实例化时每个字段都要初始化
    // 顺序不需要保持一致
    // let user = User {
    //     email: String::from("123@qq.com"),
    //     username: String::from("name123"),
    //     active: true,
    //     sign_in_count_: 1,
    // };
    change_struct();
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count_: u64,
}

fn build_user(email: String, username: String) -> User {
    return User {
        email,
        username,
        active: true,
        sign_in_count_: 1,
    };
}

// 结构体的借用权转移
fn change_struct() {
    let u1 = User {
        email: String::from("123@qq.com"),
        username: String::from("name123"),
        active: true,
        sign_in_count_: 1,
    };
    let u2 = User {
        active: u1.active,
        username: u1.username,
        email: String::from("1234@qq.com"),
        sign_in_count_: u1.sign_in_count_,
    };
    // 这行不会报错
    println!("{}", u1.active);
    // 这行会报错，因为u1有部分数据已经被重新绑定到u2
    // println!("{}", u1);
}
