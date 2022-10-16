use proconio::input;

fn main() {
    input! {
        n: i64,
        a: [i64; n],
    };

    let mut count1: i64 = 0;
    let mut count2: i64 = 0;
    let mut count3: i64 = 0;
    for a in a.iter() {
        if *a == 1 {
            count1 += 1;
        } else if *a == 2 {
            count2 += 1;
        } else if *a == 3 {
            count3 += 1;
        }
    }

    let mut answer = 0;
    answer += (count1 * (count1 - 1)) / 2;
    answer += (count2 * (count2 - 1)) / 2;
    answer += (count3 * (count3 - 1)) / 2;
    println!("{}", answer);
}
