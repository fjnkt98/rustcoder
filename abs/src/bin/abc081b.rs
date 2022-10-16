use proconio::input;

fn main() {
    input! {
        n: i32,
        mut a: [i64; n]
    };

    let mut answer = 0;
    'a: loop {
        for a in a.iter_mut() {
            if *a % 2 == 0 {
                *a /= 2;
            } else {
                break 'a;
            }
        }

        answer += 1;
    }

    println!("{}", answer);
}
