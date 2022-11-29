fn main() {
    // match_fn();
    // value_in_cents(Coin::Quarter(UsState::Alaska));
    matches();
}

enum Direction {
    East,
    West,
    North,
    South,
}

fn match_fn() {
    let dire = Direction::South;
    match dire {
        Direction::East => println!("East"),
        Direction::North | Direction::South => {
            println!("South or North")
        }
        _ => println!("West"),
    };
}

// 模式匹配的另外一个重要功能是从模式中取出绑定的值
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quater from {:?}!", state);
            25
        }
    }
}

// matches! 宏
enum MyEnum {
    Foo,
    Bar,
}

fn matches() {
    let v = vec![MyEnum::Foo, MyEnum::Bar, MyEnum::Foo];
    //报错 v.iter().filter(|x| x == MyEnum::Foo);
    v.iter().filter(|x| matches!(x, MyEnum::Foo));
}
