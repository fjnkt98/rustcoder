use proconio::input;
use std::ops;

///
/// 非再帰抽象化セグメント木
///
/// ## Attributes
///
/// * `n` - 元の配列の要素数
/// * `log` - nの二進数表現の桁数
/// * `size` - セグメント木の葉の数
/// * `op` - 二項演算子
/// * `e` - 単位元
///
#[allow(dead_code)]
struct SegmentTree<T, E, F>
where
    T: ops::Add,
    T: Copy,
    F: Fn(T, T) -> T,
    E: Fn() -> T,
{
    n: usize,
    log: usize,
    size: usize,
    op: F,
    e: E,
    data: Vec<T>,
}

#[allow(dead_code)]
impl<T, E, F> SegmentTree<T, E, F>
where
    T: ops::Add,
    T: Copy,
    F: Fn(T, T) -> T,
    E: Fn() -> T,
{
    ///
    /// コンストラクタ
    ///
    /// ## Args
    ///
    /// * `op` - 二項演算子
    /// * `e` - 単位元
    /// * `origin` - セグメント木の元になる配列
    ///
    /// ## Returns
    ///
    /// * `SegmentTree`
    ///
    pub fn new(op: F, e: E, origin: Vec<T>) -> SegmentTree<T, E, F> {
        let n = origin.len();
        // nの二進数表現の桁数を計算する
        // Rust 1.53.0からはusize::BITSとleading_zeros()を組み合わせて計算できるけど、1.42.0では無理なので
        // 2進数表現の文字列に直して長さを取得している
        let log = format!("{:b}", n - 1).len();
        let size: usize = 1 << log;

        let mut data = vec![e(); 2 * size];
        // データをコピー
        for i in 0..n {
            data[size + i] = origin[i];
        }

        // 構造体を生成
        let mut seg = SegmentTree {
            n,
            log,
            size,
            op,
            e,
            data,
        };

        // 各ブランチを更新する
        for i in (1..size).rev() {
            seg.update(i);
        }

        return seg;
    }

    /// 葉に値をセットする
    ///
    /// ## Args
    ///
    /// * `index` - 対象の葉のインデックス(0-indexed)
    /// * `value` - セットする値
    ///
    pub fn set(&mut self, index: usize, value: T) {
        assert!(index < self.size);

        // 葉に移動
        let index = index + self.size;

        // 値をセット
        self.data[index] = value;

        // 関連あるブランチを更新
        for i in 1..=self.log {
            self.update(index >> i);
        }
    }

    /// 葉の値を取得する
    ///
    /// ## Args
    ///
    /// * `index` - 対象の葉のインデックス(0-indexed)
    ///
    /// ## Returns
    ///
    /// * `T`
    ///
    pub fn get(&self, index: usize) -> T {
        assert!(index < self.size);

        return self.data[self.size + index];
    }

    /// 指定した右半開区間の積を取得する
    ///
    /// ## Args
    ///
    /// * `l` - 区間の左端(0-indexed)
    /// * `r` - 区間の右端(0-indexed) 右半開区間なのでrは含まない
    ///
    /// ## Returns
    ///
    /// * `T` - 区間の積
    ///
    pub fn prod(&self, l: usize, r: usize) -> T {
        assert!(l < r);

        // 葉に移動
        let mut l = l + self.size;
        let mut r = r + self.size;

        // 右側、左側の区間積
        let mut left_result = (self.e)();
        let mut right_result = (self.e)();

        // 指定された区間をカバーするのに必要な最小個のブランチを探す
        while l < r {
            if l & 1 == 1 {
                // lが右の子の場合、その親は選択対象とするには不適切であるので、
                // lを結果に含め、lの兄に移動する
                left_result = (self.op)(left_result, self.data[l]);

                l += 1;
            }

            if r & 1 == 1 {
                // rが右の子の場合、その親は選択対象とするには不適切であるので、
                // rを結果に含め、rの弟に移動する
                r -= 1;
                right_result = (self.op)(right_result, self.data[r]);
            }

            // 親に移動する
            l >>= 1;
            r >>= 1;
        }

        return (self.op)(left_result, right_result);
    }

    /// 子の値を用いてブランチの値を更新する
    ///
    /// ## Args
    ///
    /// * `index` - 更新対象のブランチのインデックス(0-indexed)
    ///
    fn update(&mut self, index: usize) {
        self.data[index] = (self.op)(self.data[2 * index], self.data[2 * index + 1]);
    }
}

fn main() {
    input! {
        n: usize,
        q: usize,
        query: [(usize, usize, usize); q]
    };

    let a = vec![0; n];
    let mut seg = SegmentTree::new(|a, b| std::cmp::max(a, b), || 0, a);
    for (t, x, y) in query.iter() {
        match t {
            1 => {
                let pos = x - 1;
                let x = *y;

                seg.set(pos, x as i32);
            }
            2 => {
                let l = x - 1;
                let r = y - 1;
                println!("{}", seg.prod(l, r));
            }
            _ => {}
        }
    }
}
