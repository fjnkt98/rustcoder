fn main() {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).ok();
    let line: Vec<usize> = line
        .trim()
        .split_whitespace()
        .map(|e| e.parse().ok().unwrap())
        .collect();

    let (n, q) = (line[0], line[1]);

    let mut query = vec![String::new(); q];
    for i in 0..q {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).ok();
        query[i] = line;
    }

    let mut a = vec![0; n];
    for i in 0..n {
        a[i] = i + 1;
    }

    let mut reversed = false;
    for q in query.iter() {
        let q = q.trim().split_whitespace().collect::<Vec<&str>>();
        match q[0] {
            "1" => {
                let x: usize = q[1].parse().unwrap();
                let y: usize = q[2].parse().unwrap();

                if reversed == false {
                    a[x - 1] = y;
                } else {
                    a[n - x] = y;
                }
            }
            "2" => {
                reversed = !reversed;
            }
            "3" => {
                let x: usize = q[1].parse().unwrap();
                if reversed == false {
                    println!("{}", a[x - 1]);
                } else {
                    println!("{}", a[n - x]);
                }
            }
            _ => {}
        }
    }
}
