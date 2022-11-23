use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; k]
    };

    let x = n.max(a.iter().sum());
    let mut dp = vec![false; x + 1];
    for &a in a.iter() {
        dp[a] = true;
    }

    for i in 1..=n {
        for &a in a.iter() {
            if i >= a && dp[i - a] == false {
                dp[i] = true;
            }
        }
    }

    if dp[n] {
        println!("First");
    } else {
        println!("Second");
    }
}
