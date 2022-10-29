use proconio::input;

fn main() {
    input! {
        a: i64,
        b: i64,
        c: i64,
    };

    let mut ok = false;

    if c == 1 {
        ok = false;
    } else {
        let mut r: i64 = 1;
        for _ in 0..b {
            if r > std::i64::MAX / c {
                ok = true;
                break;
            }
            r *= c;
            if a < r {
                ok = true;
                break;
            }
        }
    }

    if ok {
        println!("Yes");
    } else {
        println!("No");
    }
}
