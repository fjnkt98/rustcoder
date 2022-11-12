use proconio::input;

fn main() {
    input! {
        _: usize,
        c: char,
        a: String
    };

    let a = a
        .chars()
        .map(|x| match x {
            'W' => 0,
            'B' => 1,
            'R' => 2,
            _ => -1,
        })
        .collect::<Vec<i32>>();

    let c = match c {
        'W' => 0,
        'B' => 1,
        'R' => 2,
        _ => -1,
    };

    let score: i32 = a.iter().sum();
    if score % 3 == c {
        println!("Yes");
    } else {
        println!("No");
    }
}
