use proconio::input;

fn main() {
    input! {
        mut s: String,
    };

    loop {
        if s.ends_with("dream") {
            for _ in 0..5 {
                s.pop();
            }
        } else if s.ends_with("dreamer") {
            for _ in 0..7 {
                s.pop();
            }
        } else if s.ends_with("erase") {
            for _ in 0..5 {
                s.pop();
            }
        } else if s.ends_with("eraser") {
            for _ in 0..6 {
                s.pop();
            }
        } else {
            break;
        }
    }

    if s.len() == 0 {
        println!("YES");
    } else {
        println!("NO");
    }
}
