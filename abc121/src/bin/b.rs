use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        c: i64,
        b: [i64; m],
        a: [[i64; m]; n],
    };

    let mut answer = 0;
    for a in a.iter() {
        let mut sum = c;
        for (a, b) in a.iter().zip(b.iter()) {
            sum += a * b;
        }

        if sum > 0 {
            answer += 1;
        }
    }

    println!("{}", answer);
}
