use std::collections::VecDeque;

fn main() {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).ok();
    let q: usize = line.trim().parse().unwrap();

    let mut query: Vec<String> = vec![String::new(); q];
    for i in 0..q {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).ok();
        query[i] = line;
    }

    let mut queue: VecDeque<String> = VecDeque::new();
    for s in query.iter() {
        let query = s.trim().split_whitespace().collect::<Vec<&str>>();

        match query[0] {
            "1" => {
                let x: String = String::from(query[1]);
                queue.push_back(x);
            }
            "2" => {
                println!("{}", queue.front().unwrap());
            }
            "3" => {
                queue.pop_front();
            }
            _ => {}
        }
    }
}
