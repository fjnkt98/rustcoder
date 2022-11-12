use proconio::input;
use std::cmp;

#[derive(Clone, cmp::PartialEq, cmp::Eq, cmp::PartialOrd)]
struct State {
    score: i64,
    body: Vec<i64>,
    previous_operation: Option<bool>,
    previous_rank: Option<usize>,
    operation_count: u32,
}

impl State {
    pub fn new() -> State {
        return State {
            score: 20,
            body: vec![0; 20],
            previous_operation: None,
            previous_rank: None,
            operation_count: 0,
        };
    }

    pub fn operate(
        &self,
        operation_type: bool,
        value: (usize, usize, usize),
        rank: usize,
    ) -> State {
        let (p, q, r) = value;
        match operation_type {
            false => {
                let mut body = self.body.clone();
                body[p] += 1;
                body[q] += 1;
                body[r] += 1;

                let score = body.iter().filter(|&&x| x == 0).count() as i64;

                return State {
                    score,
                    body,
                    previous_operation: Some(operation_type),
                    previous_rank: Some(rank),
                    operation_count: self.operation_count + 1,
                };
            }

            true => {
                let mut body = self.body.clone();
                body[p] -= 1;
                body[q] -= 1;
                body[r] -= 1;

                let score = body.iter().filter(|&&x| x == 0).count() as i64;

                return State {
                    score,
                    body,
                    previous_operation: Some(operation_type),
                    previous_rank: Some(rank),
                    operation_count: self.operation_count + 1,
                };
            }
        };
    }
}

impl cmp::Ord for State {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        if self.score == other.score {
            return cmp::Ordering::Equal;
        } else if self.score > other.score {
            return cmp::Ordering::Greater;
        } else {
            return cmp::Ordering::Less;
        }
    }
}

fn main() {
    input! {
        t: usize,
        pqr: [(usize, usize, usize); t]
    };

    let mut number_of_state: Vec<usize> = vec![0; t + 1];
    number_of_state[0] = 1;
    let beam_width: usize = 12000;
    let mut beam = vec![vec![State::new(); beam_width]; t + 1];
    for i in 1..=t {
        let mut candidate: Vec<State> = Vec::new();

        let (p, q, r) = pqr[i - 1].clone();
        let values = (p - 1, q - 1, r - 1);

        for j in 0..number_of_state[i - 1] {
            let a = beam[i - 1][j].operate(false, values, j);
            let b = beam[i - 1][j].operate(true, values, j);

            candidate.push(a);
            candidate.push(b);
        }

        candidate.sort();

        number_of_state[i] = cmp::min(beam_width, candidate.len());
        for j in 0..number_of_state[i] {
            beam[i][j] = candidate.pop().unwrap();
        }
    }

    let mut current_rank = 0;
    let mut answer: Vec<String> = Vec::new();
    for i in (1..=t).rev() {
        let state = beam[i][current_rank].clone();
        answer.push(if state.previous_operation.unwrap() {
            String::from("B")
        } else {
            String::from("A")
        });
        current_rank = state.previous_rank.unwrap();
    }
    answer.reverse();

    println!("{}", answer.join("\n"));
}
