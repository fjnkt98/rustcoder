use proconio::input;

fn distance(a: (f64, f64), b: (f64, f64)) -> f64 {
    return ((a.0 - b.0).powf(2.0) + (a.1 - b.1).powf(2.0)).sqrt();
}

fn main() {
    input! {
        n: usize,
        xy: [(f64, f64); n]
    };

    let mut answer: Vec<usize> = Vec::new();
    answer.push(0);
    let mut explored = vec![false; n];
    explored[0] = true;
    let mut current = xy[0];
    for _ in 0..n {
        let mut next: usize = n;
        let mut d = std::f64::INFINITY;
        for i in 0..n {
            if explored[i] {
                continue;
            }
            if distance(current, xy[i]) < d {
                d = distance(current, xy[i]);
                next = i;
            }
        }

        if next == n {
            break;
        }

        answer.push(next);
        current = xy[next];
        explored[next] = true;
    }
    answer.push(0);

    println!(
        "{}",
        answer
            .iter()
            .map(|x| (x + 1).to_string())
            .collect::<Vec<String>>()
            .join("\n")
    );
}
