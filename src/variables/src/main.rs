fn main() {
    // let mut x = 5;
    // println!("The value of x is {}", x);
    // x = 6;
    // println!("The value of x is {}", x);
    // never_use()
    destructure()
}

// fn never_use(){
//     let _x = 5;
//     let _y = 6;
// }

fn destructure() {
    let (a, mut b): (bool, bool) = (true, false);
    println!("a = {:?}, b = {:?}", a, b);

    b = true;
    assert_eq!(a, b);
}
