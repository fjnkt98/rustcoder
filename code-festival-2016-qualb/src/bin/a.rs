use proconio::input;

fn main() {
    input! {
        s: String,
    };

    let t = "CODEFESTIVAL2016";

    let mut answer = 0;
    for (s, t) in s.chars().zip(t.chars()) {
        if s != t {
            answer += 1;
        }
    }

    println!("{}", answer);
}
