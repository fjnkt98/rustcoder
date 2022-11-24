use proconio::input;

fn main() {
    input! {
        mut x: i64,
        mut y: i64,
    };

    if x == 1 && y == 1 {
        println!("0");
        return;
    }

    let mut answer: Vec<(i64, i64)> = Vec::new();
    while !(x == 1 && y == 1) {
        answer.push((x, y));
        if x > y {
            x -= y;
        } else {
            y -= x;
        }
    }

    println!("{}", answer.len());
    println!(
        "{}",
        answer
            .iter()
            .rev()
            .map(|t| format!("{} {}", t.0, t.1))
            .collect::<Vec<String>>()
            .join("\n")
    );
}
