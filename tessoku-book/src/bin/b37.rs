use proconio::input;

fn main() {
    input! {
        n: i64,
    };

    let s = n
        .to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as i64)
        .rev()
        .collect::<Vec<i64>>();

    let mut t: Vec<Vec<i64>> = vec![vec![0; 10]; s.len()];
    for p in 0..s.len() as u32 {
        for k in 1..=9 {
            let d = s[p as usize];
            if k < d {
                t[p as usize][k as usize] = 10i64.pow(p) * (n / (10i64.pow(p + 1)) + 1);
            } else if k == d {
                t[p as usize][k as usize] =
                    10i64.pow(p) * (n / (10i64.pow(p + 1))) + n % 10i64.pow(p) + 1;
            } else {
                t[p as usize][k as usize] = 10i64.pow(p) * (n / (10i64.pow(p + 1)));
            }
        }
    }

    let mut answer = 0;
    for p in 0..s.len() {
        for k in 1..=9 {
            answer += t[p][k] * k as i64;
        }
    }
    println!("{}", answer);
}
