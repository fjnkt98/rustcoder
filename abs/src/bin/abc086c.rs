use proconio::input;

fn main() {
    input! {
        n: i32,
        txy: [(i32, i32, i32); n],
    };

    let mut t = 0;
    let mut x = 0;
    let mut y = 0;

    for (ti, xi, yi) in txy.iter() {
        let distance = (x - xi).abs() + (y - yi).abs();

        let duration = ti - t;

        if distance > duration || distance % 2 != duration % 2 {
            println!("No");
            return;
        }

        t = *ti;
        x = *xi;
        y = *yi;
    }

    println!("Yes")
}
