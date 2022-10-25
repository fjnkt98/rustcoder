use proconio::input;

fn main() {
    input! {
        h: i64,
        w: i64,
    };
    if h == 1 || w == 1 {
        println!("1");
    } else if (h * w) % 2 == 0 {
        println!("{}", (w * h) / 2);
    } else {
        println!("{}", (w * h) / 2 + 1);
    }
}
