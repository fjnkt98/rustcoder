use proconio::input;

fn main() {
    input! {
        n: i32,
        pq: [(i32, i32); n],
    };

    let mut answer = 0.0;
    for (p, q) in pq.iter() {
        answer += (*q as f64) / (*p as f64);
    }

    println!("{}", answer);
}
