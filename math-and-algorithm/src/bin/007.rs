use proconio::input;

fn main() {
    input! {
        n: i32,
        x: i32,
        y: i32,
    };

    let mut answer: i32 = 0;
    for i in 1..=n {
        if i % x == 0 || i % y == 0 {
            answer += 1;
        }
    }

    println!("{}", answer);
}
