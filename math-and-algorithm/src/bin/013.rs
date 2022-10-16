use proconio::input;

fn divisors(n: i64) -> Vec<i64> {
    let mut result: Vec<i64> = Vec::new();
    let mut i = 1;

    while i * i <= n {
        if n % i == 0 {
            result.push(i);

            if n / i != i {
                result.push(n / i);
            }
        }
        i += 1;
    }

    result.sort();

    return result;
}

fn main() {
    input! {
        n: i64,
    };

    let answers = divisors(n);
    for a in answers.iter() {
        println!("{}", a);
    }
}
