use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n]
    };

    let p = 1000000007;

    if a[0] != 0 {
        println!("0");
        return;
    }

    let mut answer = 1;
    let mut rgb = vec![-1; 3];

    for &a in a.iter() {
        answer *= rgb
            .iter()
            .filter(|&&x| x == a - 1)
            .collect::<Vec<&i64>>()
            .len();
        if let Some(i) = rgb.iter().position(|x| *x == a - 1) {
            rgb[i] = a;
        } else {
            println!("0");
            return;
        }

        answer %= p;
    }

    println!("{}", answer);
}
