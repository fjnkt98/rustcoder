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

    let mut stack: Vec<String> = Vec::new();
    for s in query.iter() {
        let query = s.trim().split_whitespace().collect::<Vec<&str>>();

        match query[0] {
            "1" => {
                let x: String = String::from(query[1]);
                stack.push(x);
            }
            "2" => {
                println!("{}", stack.last().unwrap());
            }
            "3" => {
                stack.pop();
            }
            _ => {}
        }
    }
}
