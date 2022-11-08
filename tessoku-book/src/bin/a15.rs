use proconio::input;
use std::collections::HashMap;
use std::hash::Hash;

fn compress1d<T>(a: &Vec<T>) -> Vec<usize>
where
    T: Ord,
    T: Clone,
    T: Hash,
    T: Copy,
{
    let mut x = a.to_vec();
    x.sort();
    x.dedup();

    let mut d: HashMap<T, usize> = HashMap::new();
    for (i, &x) in x.iter().enumerate() {
        d.insert(x, i);
    }

    let mut b = vec![0; a.len()];
    for (i, a) in a.iter().enumerate() {
        b[i] = *d.get(a).unwrap();
    }

    return b;
}

fn main() {
    input! {
        n: usize,
        a: [i64; n]
    };

    let b = compress1d(&a);
    println!(
        "{}",
        b.iter()
            .map(|x| (x + 1).to_string())
            .collect::<Vec<String>>()
            .join(" ")
    )
}
