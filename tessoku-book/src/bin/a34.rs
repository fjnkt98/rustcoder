use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
        y: usize,
        a: [usize; n]
    };

    let max: usize = *a.iter().max().unwrap() as usize;
    let mut g: Vec<usize> = vec![0; max + 1];
    for i in 1..=max {
        let mut t = [false, false, false];
        if i >= x {
            t[g[i - x]] = true;
        }
        if i >= y {
            t[g[i - y]] = true;
        }

        if t[0] == false {
            g[i] = 0;
        } else if t[1] == false {
            g[i] = 1;
        } else {
            g[i] = 2;
        }
    }

    let mut sum = 0;
    for &a in a.iter() {
        sum ^= g[a];
    }
    if sum == 0 {
        println!("Second");
    } else {
        println!("First");
    }
}
