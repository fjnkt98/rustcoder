use proconio::input;

fn main() {
    input! {
        n: usize,
        a: usize,
        b: usize,
        s: String,
    };

    let mut count = 0;
    let mut foreign = 0;
    let mut passed = vec![false; n];
    for (i, c) in s.chars().enumerate() {
        if c == 'a' {
            if count < a + b {
                passed[i] = true;
                count += 1;
            }
        } else if c == 'b' {
            if count < a + b && foreign < b {
                passed[i] = true;
                count += 1;
            }
            foreign += 1;
        }
    }

    println!(
        "{}",
        passed
            .iter()
            .map(|x| if *x { "Yes" } else { "No" })
            .collect::<Vec<&str>>()
            .join("\n")
    );
}
