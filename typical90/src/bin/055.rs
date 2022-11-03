use proconio::input;

fn main() {
    input! {
        n: usize,
        p: u64,
        q: u64,
        a: [u64; n]
    };

    let mut answer = 0;
    for i in 0..n {
        for j in i + 1..n {
            for k in j + 1..n {
                for l in k + 1..n {
                    for m in l + 1..n {
                        let mut prod = 1;
                        prod *= a[i];
                        prod %= p;
                        prod *= a[j];
                        prod %= p;
                        prod *= a[k];
                        prod %= p;
                        prod *= a[l];
                        prod %= p;
                        prod *= a[m];
                        prod %= p;

                        if prod == q {
                            answer += 1;
                        }
                    }
                }
            }
        }
    }

    println!("{}", answer);
}
