fn main() {
    // 实例化时每个字段都要初始化
    // 顺序不需要保持一致
    // let user = User {
    //     email: String::from("123@qq.com"),
    //     username: String::from("name123"),
    //     active: true,
    //     sign_in_count_: 1,
    // };
    // change_struct();
    // no_lifetimes();
    test_debug();
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

// 执行下面的代码，编译器会提示需要生命周期
// 原因是 UserOne 结构体中的数据是 “借用” 的
// struct UserOne {
//     username: &str,
//     email: &str,
// }

// fn no_lifetimes() {
//     let u1 = UserOne {
//         username: "Ho",
//         email: "123@qq.com",
//     };
// }


#[derive(Debug)]
struct Rect {
    width: u32,
    height: u32,
}

// 打印结构体的方式
fn test_debug(){
    let r1 = Rect{
        width:200,
        height:50
    };
    // 打印结构体的方式一，在结构体上使用 derive，然后通过 {:#?} 打印
    // println!("r1 is {:#?}", r1);
    // 打印结构体的方式二：在结构体上使用 derive，使用 dbg! 宏 打印
    dbg!(&r1);
}
