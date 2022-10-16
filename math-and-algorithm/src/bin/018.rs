use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: i64,
        a: [i64; n],
    };

    let mut counter: HashMap<i64, i64> = HashMap::new();
    counter.insert(100, 0);
    counter.insert(200, 0);
    counter.insert(300, 0);
    counter.insert(400, 0);
    for a in a.iter() {
        let e = counter.entry(*a).or_insert(0);
        *e += 1;
    }

    let answer = counter.get(&100).unwrap() * counter.get(&400).unwrap()
        + counter.get(&200).unwrap() * counter.get(&300).unwrap();
    println!("{}", answer);
}
