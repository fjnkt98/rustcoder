use itertools::iproduct;
use num;
use proconio::input;
use std::cmp;
use std::ops;

/// # 2次元累積和
///
/// ## Attributes
///
/// * `h` - グリッドの行数
/// * `w` - グリッドの列数
/// * `body` - グリッドの実体
///
struct CumulativeSum2D<T> {
    h: usize,
    w: usize,
    body: Vec<Vec<T>>,
}

impl<T> CumulativeSum2D<T>
where
    T: ops::Add,
    T: ops::AddAssign,
    T: ops::Sub,
    T: ops::SubAssign,
    T: num::Zero,
    T: Clone,
    T: Copy,
{
    ///
    /// # コンストラクタ
    ///
    /// ## Args
    ///
    /// * `h` - グリッドの行数
    /// * `w` - グリッドの列数
    ///
    pub fn new(h: usize, w: usize) -> CumulativeSum2D<T> {
        let body: Vec<Vec<T>> = vec![vec![T::zero(); h + 1]; w + 1];

        return CumulativeSum2D { h, w, body };
    }

    ///
    /// # グリッドに値をセットする
    ///
    /// ## Args
    ///
    /// * `r` - 値をセットするセルの行
    /// * `c` - 値をセットするセルの列
    /// * `t` - セットする値
    ///
    pub fn set(&mut self, r: usize, c: usize, t: T) {
        self.body[r][c] = t;
    }

    /// # 累積和を計算する
    ///
    /// 計算量はO(hw)
    ///
    pub fn build(&mut self) {
        for i in 0..=self.h {
            for j in 0..=self.w {
                let mut sum = T::zero();
                if i > 0 {
                    sum += self.body[i - 1][j];
                }
                if j > 0 {
                    sum += self.body[i][j - 1];
                }
                if i > 0 && j > 0 {
                    sum -= self.body[i - 1][j - 1];
                }

                self.body[i][j] += sum;
            }
        }
    }

    /// # グリッドの領域和を求める関数
    ///
    /// 領域の左上と右下を指定することで、その領域の領域和をO(1)で求める。
    /// フラグ変数`right_open`を設定することにより、右半開区間か左半開区間かを指定することができる。
    ///
    /// `right_open`が`true`のときは右半開区間で領域和を計算するので、左上のセルを含み、右下のセルは含まない。
    /// `right_open`が`false`のときは左半開区間で領域和を計算するので、左上のセルは含まず、右下のセルを含む。
    ///
    /// ## Args
    ///
    /// * `start` - 求める領域の左上のセル
    /// * `end` - 求める領域の右下のセル
    ///
    /// ## Returns
    ///
    /// * 領域和
    ///
    pub fn get(&self, start: (usize, usize), end: (usize, usize), right_open: bool) -> T {
        let (sr, sc) = start;
        let (er, ec) = end;

        assert!(sr < er && sc < ec);

        match right_open {
            true => {
                let mut result = T::zero();

                result += self.body[er - 1][ec - 1];

                if sr > 0 {
                    result -= self.body[sr - 1][ec - 1];
                }
                if sc > 0 {
                    result -= self.body[er - 1][sc - 1];
                }
                if sr > 0 && sc > 0 {
                    result += self.body[sr - 1][sc - 1];
                }

                return result;
            }
            false => {
                let mut result = T::zero();

                result += self.body[er][ec];
                result -= self.body[sr][ec];
                result -= self.body[er][sc];
                result += self.body[sr][sc];

                return result;
            }
        }
    }
}

fn main() {
    input! {
        n: usize,
        d: [[i64; n]; n],
        q: usize,
        p: [i64; q]
    };

    let mut c: CumulativeSum2D<i64> = CumulativeSum2D::new(n, n);
    for i in 0..n {
        for j in 0..n {
            c.set(i, j, d[i][j]);
        }
    }

    c.build();

    let mut area: Vec<i64> = vec![0; n * n + 1];
    for (i, j) in iproduct!(0..n, 0..n) {
        for (k, l) in iproduct!(i + 1..=n, j + 1..=n) {
            let a = (k - i) * (l - j);
            let sum = c.get((i, j), (k, l), true);

            if area[a] < sum {
                area[a] = sum;
            }
        }
    }

    let mut i = 0;
    while i < n * n {
        let a = cmp::max(area[i + 1], area[i]);
        area[i + 1] = a;
        i += 1;
    }

    for &p in p.iter() {
        println!("{}", area[p as usize]);
    }
}
