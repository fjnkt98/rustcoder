use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        sc: [(usize, char); m]
    };

    match n {
        1 => {
            for i in 0..=9 {
                let x = i.to_string().chars().collect::<Vec<char>>();
                let mut ok = true;
                for &(s, c) in sc.iter() {
                    if x[s - 1] != c {
                        ok = false;
                    }
                }

                if ok {
                    println!("{}", i);
                    return;
                }
            }
            println!("-1");
        }
        2 => {
            for i in 10..=99 {
                let x = i.to_string().chars().collect::<Vec<char>>();
                let mut ok = true;
                for &(s, c) in sc.iter() {
                    if x[s - 1] != c {
                        ok = false;
                    }
                }

                if ok {
                    println!("{}", i);
                    return;
                }
            }
            println!("-1");
        }
        3 => {
            for i in 100..=999 {
                let x = i.to_string().chars().collect::<Vec<char>>();
                let mut ok = true;
                for &(s, c) in sc.iter() {
                    if x[s - 1] != c {
                        ok = false;
                    }
                }

                if ok {
                    println!("{}", i);
                    return;
                }
            }
            println!("-1");
        }
        _ => {}
    }
}
