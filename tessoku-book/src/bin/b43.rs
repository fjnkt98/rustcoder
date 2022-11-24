use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; m]
    };

    let mut c = vec![m; n];
    for &a in a.iter() {
        c[a - 1] -= 1;
    }

    println!(
        "{}",
        c.iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join("\n")
    );
}
