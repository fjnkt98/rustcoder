use proconio::input;

fn main() {
    input! {
        n: usize,
        l: i64,
        ab: [(i64, char); n]
    };

    let mut answer = 0;
    for &(a, b) in ab.iter() {
        let distance = match b {
            'E' => l - a,
            'W' => a,
            _ => 0,
        };

        answer = std::cmp::max(answer, distance);
    }

    println!("{}", answer);
}
