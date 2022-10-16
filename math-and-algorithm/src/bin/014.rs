use proconio::input;

fn factorize(n: i64) -> Vec<i64> {
    let mut n = n;
    let mut result: Vec<i64> = Vec::new();

    let mut i = 2;
    while i * i <= n {
        while n % i == 0 {
            n /= i;
            result.push(i);
        }

        i += 1;
    }

    if n != 1 {
        result.push(n);
    }

    result.sort();

    return result;
}

fn main() {
    input! {
        n: i64,
    };

    let answers = factorize(n);
    let mut answer = String::new();
    for a in answers.iter() {
        answer.push_str(" ");
        answer.push_str(&(a.to_string()));
    }
    println!("{}", answer.trim());
}
