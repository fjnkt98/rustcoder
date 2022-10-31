use proconio::input;

fn main() {
    input! {
        mut a: [[i64; 3]; 3],
        n: usize,
        b: [i64; n]
    };

    let mut bingo = vec![vec![false; 3]; 3];
    for &b in b.iter() {
        for i in 0..3 {
            for j in 0..3 {
                if a[i][j] == b {
                    bingo[i][j] = true;
                }
            }
        }
    }

    let mut ok = false;
    for i in 0..3 {
        if bingo[i][0] && bingo[i][1] && bingo[i][2] {
            ok = true;
        }
    }
    for j in 0..3 {
        if bingo[0][j] && bingo[1][j] && bingo[2][j] {
            ok = true;
        }
    }

    if bingo[0][0] && bingo[1][1] && bingo[2][2] {
        ok = true;
    }
    if bingo[0][2] && bingo[1][1] && bingo[2][0] {
        ok = true;
    }

    if ok {
        println!("Yes");
    } else {
        println!("No");
    }
}
