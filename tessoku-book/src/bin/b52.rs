use proconio::input;
use std::collections::VecDeque;

fn main() {
    input! {
        n: usize,
        x: usize,
        a: String
    };

    let x = x - 1;
    let mut a = a.chars().collect::<Vec<char>>();
    let mut queue: VecDeque<usize> = VecDeque::new();
    queue.push_back(x);
    a[x] = '@';
    while let Some(p) = queue.pop_front() {
        if p > 0 && a[p - 1] == '.' {
            a[p - 1] = '@';
            queue.push_back(p - 1);
        }
        if p < n - 1 && a[p + 1] == '.' {
            a[p + 1] = '@';
            queue.push_back(p + 1);
        }
    }

    println!(
        "{}",
        a.iter()
            .map(|c| c.to_string())
            .collect::<Vec<String>>()
            .join("")
    );
}
