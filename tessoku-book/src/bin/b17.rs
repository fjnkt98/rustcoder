use proconio::input;

fn main() {
    input! {
        n: usize,
        h: [i64; n]
    };

    let mut dp: Vec<i64> = vec![0; n];
    let mut prev: Vec<usize> = vec![0; n];
    dp[0] = 0;
    dp[1] = (h[1] - h[0]).abs();
    prev[1] = 0;

    for i in 2..n {
        let a = dp[i - 1] + (h[i] - h[i - 1]).abs();
        let b = dp[i - 2] + (h[i] - h[i - 2]).abs();

        match a.cmp(&b) {
            std::cmp::Ordering::Equal | std::cmp::Ordering::Greater => {
                dp[i] = b;
                prev[i] = i - 2;
            }
            std::cmp::Ordering::Less => {
                dp[i] = a;
                prev[i] = i - 1;
            }
        }
    }

    let mut answer: Vec<usize> = Vec::new();
    let mut current: usize = n - 1;
    while current != 0 {
        answer.push(current);
        current = prev[current];
    }
    answer.push(0);
    answer.reverse();

    println!("{}", answer.len());
    println!(
        "{}",
        answer
            .iter()
            .map(|x| (x + 1).to_string())
            .collect::<Vec<String>>()
            .join(" ")
    );
}
