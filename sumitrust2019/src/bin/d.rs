use proconio::input;

fn main() {
    input! {
        n: i64,
        s: String
    };

    let mut answer = 0;
    for i in 0..=999 {
        let mut t = i.to_string();
        if t.len() == 1 {
            t = String::from("00") + &t;
        } else if t.len() == 2 {
            t = String::from("0") + &t;
        }

        let t = t.chars().collect::<Vec<char>>();

        let mut i = 0;
        for c in s.chars() {
            if c == t[i] {
                i += 1;
            }

            if i >= 3 {
                break;
            }
        }

        if i == 3 {
            answer += 1;
        }
    }

    println!("{}", answer);
}
