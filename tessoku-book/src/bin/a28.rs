use proconio::input;

fn main() {
    input! {
        n: usize,
        ta: [(char, i64); n]
    };

    let p = 10000;

    let mut answer = 0;
    for &(t, a) in ta.iter() {
        match t {
            '+' => {
                answer += a;
                answer %= p;
                println!("{}", answer);
            }
            '-' => {
                answer -= a;
                if answer < 0 {
                    answer += p;
                }
                answer %= p;
                println!("{}", answer);
            }
            '*' => {
                answer *= a;
                answer %= p;
                println!("{}", answer);
            }
            _ => {}
        }
    }
}
