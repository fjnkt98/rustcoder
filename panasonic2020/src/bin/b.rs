use proconio::input;

fn main() {
    input! {
        h: i64,
        w: i64
    };

    if h == 1 || w == 1 {
        println!("1");
    } else {
        if h % 2 == 1 && w % 2 == 1 {
            println!("{}", h * w / 2 + 1);
        } else {
            println!("{}", h * w / 2);
        }
    }
}
