use std::collections::BTreeSet;

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

    let mut set = BTreeSet::new();
    for s in query.iter() {
        let query = s.trim().split_whitespace().collect::<Vec<&str>>();

        match query[0] {
            "1" => {
                let x: i64 = query[1].parse().unwrap();
                set.insert(x);
            }
            "2" => {
                let x: i64 = query[1].parse().unwrap();
                set.remove(&x);
            }
            "3" => {
                let x: i64 = query[1].parse().unwrap();
                if let Some(answer) = set.range(x..).next() {
                    println!("{}", answer);
                } else {
                    println!("{}", -1);
                }
            }
            _ => {}
        }
    }
}
