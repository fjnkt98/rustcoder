use proconio::input;

fn main() {
    input! {
        s: String
    };

    let mut answer = 0;
    for c in s.chars() {
        if c == '1' {
            answer += 1;
        }
    }

    println!("{}", answer);
}
