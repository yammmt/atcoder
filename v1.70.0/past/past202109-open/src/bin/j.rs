use proconio::fastout;
use proconio::input;

#[derive(Debug)]
struct SegTree<T, F>
where
    T: Clone,
    F: Fn(&T, &T) -> T,
{
    n: usize,
    size: usize,
    node: Vec<T>,
    /// 単位元
    identity_e: T,
    /// 2 つのデータから値を合成するための関数
    combine_f: F,
}

impl<T, F> SegTree<T, F>
where
    T: Clone,
    F: Fn(&T, &T) -> T,
{
    fn new(n: usize, identity_e: T, combine_f: F) -> Self {
        let mut size = 1;
        while size < n {
            size *= 2;
        }
        let node = vec![identity_e.clone(); 2 * size];

        SegTree {
            n,
            size,
            node,
            identity_e,
            combine_f,
        }
    }

    /// 配列の各要素を登録する
    fn build(&mut self, array: &[T]) {
        assert_eq!(array.len(), self.n);
        for (i, &ref a) in array.iter().enumerate() {
            self.node[i + self.size] = a.clone();
        }
        // 0 はダミーノードだから回さなくてよい
        // ノード i の左を 2i, 右を 2i+1 で表すために好都合
        for i in (1..self.size).rev() {
            // {左,右} の子
            self.node[i] = (self.combine_f)(&self.node[i << 1 | 0], &self.node[i << 1 | 1]);
        }
    }

    /// [一点更新] 位置を値で更新
    fn update(&mut self, index: usize, value: T) {
        let mut i = self.size + index;
        self.node[i] = value;
        while i > 1 {
            i >>= 1;
            self.node[i] = (self.combine_f)(&self.node[i << 1 | 0], &self.node[i << 1 | 1]);
        }
    }

    /// [区間取得] 区間 [l, r) 内の `combine_f` を順々に適用した結果
    fn fold(&self, mut l: usize, mut r: usize) -> T {
        l += self.size;
        r += self.size;
        let mut value_l = self.identity_e.clone();
        let mut value_r = self.identity_e.clone();
        while l < r {
            if l & 1 > 0 {
                value_l = (self.combine_f)(&value_l, &self.node[l]);
                l += 1;
            }
            if r & 1 > 0 {
                r -= 1;
                value_r = (self.combine_f)(&self.node[r], &value_r);
            }
            l >>= 1;
            r >>= 1;
        }
        (self.combine_f)(&value_l, &value_r)
    }
}

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        tkq: [(usize, usize); q],
    }

    // 左から j 番目/2n-j+1 番目の要素, 反転の組となるものをまとめて考える.
    // 反転クエリでは最初の k 枚を反転させればよい. として一点加算区間和問題

    let mut st = SegTree::new(n, 0, |&a, &b| a + b);
    for (t, k) in tkq {
        if t == 1 {
            // セグメント木上で何番目に相当する要素かを求める
            let pos = if k <= n { n + 1 - k } else { k - n };
            let parity = (st.fold(pos - 1, n) + if k > n { 1 } else { 0 }) % 2;
            println!("{}", if parity == 0 { n + 1 - pos } else { n + pos });
        } else {
            // ひっくり返す, 偶奇が変化
            // ここの `fold` は k-1 一点の値を取得するもの
            let sum_value = st.fold(k - 1, k);
            st.update(k - 1, sum_value + 1);
        }
    }
}
