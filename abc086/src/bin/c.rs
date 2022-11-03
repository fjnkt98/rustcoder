use proconio::input;

fn main() {
    input! {
        n: usize,
        txy: [(i64, i64, i64); n],
    };

    let mut current_point = (0, 0);
    let mut current_time = 0;
    let mut ok = true;
    for &(t, nx, ny) in txy.iter() {
        let (x, y) = current_point;

        let distance = (x - nx).abs() + (y - ny).abs();
        let duration = t - current_time;

        if distance > duration || duration % 2 != distance % 2 {
            ok = false;
            break;
        }

        current_point = (nx, ny);
        current_time = t;
    }

    if ok {
        println!("Yes");
    } else {
        println!("No");
    }
}
