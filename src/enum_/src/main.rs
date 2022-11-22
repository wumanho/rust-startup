fn main() {
    // enum_fn();
    let some_number = Some(5);
    // 使用 None 的时候，需要指定 Option 的泛型值
    let absent_number:Option<i32> = None;
    println!("{:?}", some_number);
}

// 使用枚举定义扑克牌案例一
// enum PokerSuit {
//     Clubs,
//     Spades,
//     Diamonds,
//     Headrts,
// }

// struct PokerCard {
//     suit: PokerSuit,
//     value: u8,
// }

// fn enum_fn() {
//     let c1 = PokerCard {
//         suit: PokerSuit::Clubs,
//         value: 8,
//     };
//     let c2 = PokerCard {
//         suit: PokerCard::Headrts,
//         value: 5,
//     };
// }

// fn print_suit(card: PokerSuit) {
//     println!("{:?}", card)
// }

// 使用枚举定义扑克牌案例二
// enum 可以直接指定数据类型，实例化时跟数据绑定
enum PokerCard {
    Clubs(u8),
    Spades(u8),
    Diamonds(char),
    Hearts(char),
}

// 省去了结构体声明
fn enum_fn() {
    let c1 = PokerCard::Clubs(5);
    let c2 = PokerCard::Hearts('A');
}
