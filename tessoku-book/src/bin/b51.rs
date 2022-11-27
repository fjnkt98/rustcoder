use proconio::input;

fn main() {
    input! {
        s: String,
    };

    let mut stack: Vec<usize> = Vec::new();
    let mut answer: Vec<(usize, usize)> = Vec::new();
    for (i, c) in s.chars().enumerate() {
        if c == '(' {
            stack.push(i + 1);
        } else {
            answer.push((stack.pop().unwrap(), i + 1));
        }
    }

    answer.sort_by_key(|x| std::cmp::max(x.0, x.1));

    println!(
        "{}",
        answer
            .iter()
            .map(|x| format!("{} {}", x.0, x.1))
            .collect::<Vec<String>>()
            .join("\n")
    );
}
