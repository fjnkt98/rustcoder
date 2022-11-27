use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [[i64; n]; n],
        q: usize,
        txy: [(u8, usize, usize); q]
    };

    let mut map: Vec<usize> = vec![0; n];
    for i in 0..n {
        map[i] = i;
    }

    for (t, x, y) in txy.iter() {
        let x = *x - 1;
        let y = *y - 1;
        if *t == 1 {
            map.swap(x, y);
        } else {
            println!("{}", a[map[x]][y]);
        }
    }
}
