use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        s: [String; n]
    };

    let mut set = HashSet::new();
    let mut answer = Vec::new();
    for (i, s) in s.iter().enumerate() {
        if set.contains(s) {
            continue;
        } else {
            set.insert(s);
            answer.push(i + 1);
        }
    }

    println!(
        "{}",
        answer
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join("\n")
    );
}
