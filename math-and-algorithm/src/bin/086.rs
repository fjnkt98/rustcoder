use proconio::input;

fn main() {
    input! {
        n: usize,
        s: String
    };

    let mut count = 0;
    for c in s.chars() {
        if c == '(' {
            count += 1;
        } else if c == ')' {
            count -= 1;
        }

        if count < 0 {
            println!("No");
            return;
        }
    }

    println!("Yes");
}
