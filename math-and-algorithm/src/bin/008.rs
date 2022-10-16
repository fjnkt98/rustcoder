use proconio::input;

fn main() {
    input! {
        n: i32,
        s: i32,
    };

    let mut answer: i32 = 0;
    for i in 1..=n {
        for j in 1..=n {
            if i + j <= s {
                answer += 1;
            }
        }
    }

    println!("{}", answer);
}
