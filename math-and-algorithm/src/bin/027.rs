use proconio::input;

fn main() {
    input! {
        n: i64,
        mut a: [i64; n]
    };

    merge_sort(&mut a, 0, n);

    println!(
        "{}",
        a.iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    );
}

fn merge_sort(a: &mut [i64], l: i64, r: i64) {
    if r - l == 1 {
        return;
    }

    let m = (r + l) / 2;
    merge_sort(a, l, m);
    merge_sort(a, m, r);

    let mut c1 = l;
    let mut c2 = m;
    let mut count: i64 = 0;
    let mut b = Vec::<i64>::new();

    while c1 != m || c2 != r {
        if c1 == m {
            b.push(a[c2 as usize]);
            c2 += 1;
        } else if c2 == r {
            b.push(a[c1 as usize]);
            c1 += 1;
        } else {
            if a[c1 as usize] < a[c2 as usize] {
                b.push(a[c1 as usize]);
                c1 += 1;
            } else {
                b.push(a[c2 as usize]);
                c2 += 1;
            }
        }

        count += 1;
    }

    for i in 0..count {
        a[(l + i) as usize] = b[i as usize];
    }
}
