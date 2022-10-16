use proconio::input;

fn main() {
    input! {
        n: i64,
        a: i64,
        b: i64,
    };

    let mut answers: Vec<i64> = Vec::new();
    for i in 1..=n {
        let mut sum = 0;
        for c in i.to_string().chars() {
            sum += c.to_digit(10).unwrap() as i64;
        }

        if a <= sum && sum <= b {
            answers.push(i);
        };
    }

    println!("{}", answers.iter().sum::<i64>());
}
