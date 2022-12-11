use proconio::input;
use std::collections::HashMap;

fn f(x: i64, memo: &mut HashMap<i64, i64>) -> i64 {
    if x == 0 {
        return 1;
    }

    if let Some(v) = memo.get(&x) {
        return *v;
    } else {
        let v = f(x / 2, memo) + f(x / 3, memo);
        memo.insert(x, v);
        return v;
    }
}

fn main() {
    input! {
        n: i64,
    };

    let mut memo: HashMap<i64, i64> = HashMap::new();

    println!("{}", f(n, &mut memo));
}
