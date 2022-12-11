use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n]
    };

    let mut b = vec![0; 2 * n + 2];
    b[1] = 0;
    for (i, &a) in a.iter().enumerate() {
        let i = i + 1;

        b[2 * i] = b[a] + 1;
        b[2 * i + 1] = b[a] + 1;
    }
    println!(
        "{}",
        b[1..]
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join("\n")
    );
}
