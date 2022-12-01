use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("猜数字游戏");
    let target = rand::thread_rng().gen_range(1, 101);

    loop { 
        println!("请输入数字：");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("读取失败");
        println!("你猜的数字：{}", guess);
        let guess: u32 = guess.trim().parse().expect("类型转换错误");
        match guess.cmp(&target) {
            Ordering::Less => println!("小了"),
            Ordering::Greater => println!("大了"),
            Ordering::Equal => {
                println!("猜中了");
                println!("正确的数字：{}", target);
                break;
            }
            _ => println!("错误"),
        }
    }
}
