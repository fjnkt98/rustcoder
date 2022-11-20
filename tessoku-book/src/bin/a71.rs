use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [i64; n],
        mut b: [i64; n],
    };

    a.sort();
    b.sort_by_key(|x| std::cmp::Reverse(*x));

    let mut answer = 0;
    for (a, b) in a.iter().zip(b.iter()) {
        answer += a * b;
    }

    println!("{}", answer);
}
