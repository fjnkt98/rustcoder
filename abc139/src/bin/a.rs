use proconio::input;

fn main() {
    input! {
        s: String,
        t: String,
    };

    let mut answer = 0;
    for (s, t) in s.chars().zip(t.chars()) {
        if s == t {
            answer += 1;
        }
    }

    println!("{}", answer);
}
