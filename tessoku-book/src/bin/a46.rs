use itertools::Itertools;
use proconio::input;
use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};
use std::time;

#[allow(dead_code)]
struct Answer {
    pub p: Vec<usize>,
    pub total_distance: f64,
    pub score: i64,
}

impl Answer {
    pub fn new(p: &Vec<usize>, xy: &Vec<(f64, f64)>) -> Answer {
        let mut total_distance = 0.0;
        for (&i, &j) in p.iter().tuple_windows() {
            total_distance +=
                ((xy[i].0 - xy[j].0).powf(2.0) + (xy[i].1 - xy[j].1).powf(2.0)).sqrt();
        }

        let score = (1000000.0 / total_distance) as i64;
        return Answer {
            p: p.clone(),
            total_distance,
            score,
        };
    }
}

fn main() {
    input! {
        n: usize,
        xy: [(f64, f64); n]
    };

    let now = time::Instant::now();

    // ランダムな順列を生成
    let mut initial_value = (1..n).collect::<Vec<usize>>();
    let mut rng = thread_rng();
    initial_value.shuffle(&mut rng);

    // 初期解を生成
    let mut answer: Vec<usize> = Vec::new();
    answer.push(0);
    answer.append(&mut initial_value);
    answer.push(0);
    let mut current_answer = Answer::new(&answer, &xy);

    let time = 990.0;
    let t_start = 30.0;
    let t_end = 2.0;

    while now.elapsed() < time::Duration::from_millis(time as u64) {
        // 入れ替え元
        let mut l = rng.gen_range(1, n + 1);
        let mut r = rng.gen_range(1, n + 1);

        if l > r {
            std::mem::swap(&mut l, &mut r);
        }

        if r - l <= 1 {
            continue;
        }

        let mut p = current_answer.p.clone();
        p[l..r].reverse();

        let new_answer = Answer::new(&p, &xy);

        let t: f64 = (now.elapsed().as_millis() as f64 * (t_end - t_start)) / time + t_start;
        // let delta = new_answer.score - current_answer.score;
        let delta = current_answer.total_distance - new_answer.total_distance;
        if delta > 0.0 {
            current_answer = new_answer;
        } else {
            let p = (delta as f64 / t).exp();
            if rng.gen_range(0.0, 1.0) < p {
                current_answer = new_answer;
            }
        }
    }

    println!(
        "{}",
        current_answer
            .p
            .iter()
            .map(|x| (x + 1).to_string())
            .collect::<Vec<String>>()
            .join("\n")
    );
    // println!("{:?}", current_answer.p);
    // println!(
    //     "total_distance: {}, score: {}",
    //     current_answer.total_distance, current_answer.score
    // );
}
