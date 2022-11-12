use itertools::Itertools;
use proconio::input;
use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};
use std::time;

fn distance(a: &(f64, f64), b: &(f64, f64)) -> f64 {
    return ((a.0 - b.0).powf(2.0) + (a.1 - b.1).powf(2.0)).sqrt();
}

fn score(p: &[usize], xy: &[(f64, f64)]) -> f64 {
    let mut sum = 0.0;
    for (&i, &j) in p.iter().tuple_windows() {
        sum += distance(&xy[i], &xy[j]);
    }
    return sum;
}

fn main() {
    input! {
        n: usize,
        xy: [(f64, f64); n]
    };

    let now = time::Instant::now();

    let mut initial_value = (1..n).collect::<Vec<usize>>();
    let mut rng = thread_rng();
    initial_value.shuffle(&mut rng);

    let mut answer: Vec<usize> = Vec::new();
    answer.push(0);
    answer.append(&mut initial_value);
    answer.push(0);

    let mut current_score = score(&answer, &xy);
    while now.elapsed() < time::Duration::from_millis(990) {
        let mut l = rng.gen_range(1, n + 1);
        let mut r = rng.gen_range(1, n + 1);

        if l > r {
            std::mem::swap(&mut l, &mut r);
        }

        if r - l <= 1 {
            continue;
        }

        let mut new_answer = answer.clone();
        new_answer[l..r].reverse();

        let new_score = score(&new_answer, &xy);

        let t: f64 = -(now.elapsed().as_millis() as f64 * 28.0) / 990.0 + 30.0;
        let delta = current_score - new_score;
        if delta >= 0.0 {
            answer = new_answer;
            current_score = new_score;
        } else {
            let p = (delta / t).exp();
            if rng.gen_range(0.0, 1.0) < p {
                answer = new_answer;
                current_score = new_score;
            }
        }
    }

    println!(
        "{}",
        answer
            .iter()
            .map(|x| (x + 1).to_string())
            .collect::<Vec<String>>()
            .join("\n")
    );
}
