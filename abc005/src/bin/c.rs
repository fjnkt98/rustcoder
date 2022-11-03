use proconio::input;

fn main() {
    input! {
        t: i64,
        n: usize,
        a: [i64; n],
        m: usize,
        b: [i64; m],
    };

    let mut ok = true;
    let mut sold = vec![false; n];
    for &b in b.iter() {
        let mut available = false;
        for (i, &a) in a.iter().enumerate() {
            if sold[i] {
                continue;
            }
            if b - a >= 0 && b - a <= t {
                available = true;
                sold[i] = true;
                break;
            }
        }

        if !available {
            ok = false;
            break;
        }
    }

    if ok {
        println!("yes");
    } else {
        println!("no");
    }
}
