use proconio::input;

fn main() {
    input! {
        w: usize,
        h: usize,
        p: [i64; w],
        q: [i64; h],
    };

    let mut r: Vec<(i64, bool)> = Vec::with_capacity(w + h);
    for &p in p.iter() {
        r.push((p, true));
    }
    for &q in q.iter() {
        r.push((q, false));
    }

    r.sort();

    let mut a = w as i64 + 1;
    let mut b = h as i64 + 1;

    let mut answer: i64 = 0;
    for &(r, is_p) in r.iter() {
        if is_p {
            answer += r * b;
            a -= 1;
        } else {
            answer += r * a;
            b -= 1;
        }
    }

    println!("{}", answer);
}
