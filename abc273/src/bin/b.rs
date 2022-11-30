use proconio::input;

fn d(x: i64, k: u32) -> i64 {
    return x % 10i64.pow(k + 1) / 10i64.pow(k);
}

fn main() {
    input! {
        mut x: i64,
        k: u32
    };

    // let s = x
    //     .to_string()
    //     .chars()
    //     .map(|x| x as i64 - 48)
    //     .collect::<Vec<i64>>();

    for i in 0..k {
        if d(x, i) >= 5 {
            x = (x / 10i64.pow(i + 1)) * 10i64.pow(i + 1) + 10i64.pow(i + 1);
        } else {
            x = (x / 10i64.pow(i + 1)) * 10i64.pow(i + 1);
        }
    }
    println!("{}", x);
}
