use proconio::input;

fn main() {
    input! {
        n: i64,
        a: [i64; n],
    };

    let mut answer: i64 = 0;
    for i in 0..n {
        for j in i + 1..n {
            for k in j + 1..n {
                for l in k + 1..n {
                    for m in l + 1..n {
                        if a[i as usize]
                            + a[j as usize]
                            + a[k as usize]
                            + a[l as usize]
                            + a[m as usize]
                            == 1000
                        {
                            answer += 1;
                        }
                    }
                }
            }
        }
    }

    println!("{}", answer);
}
