use proconio::fastout;
use proconio::input;
use std::collections::HashMap;

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
        l: usize,
        q: usize,
        cxq: [(usize, usize); q],
    }
    let mut positions = cxq.iter().map(|&cx| cx.1).collect::<Vec<usize>>();
    positions.sort_unstable();
    positions.dedup();
    let n = positions.len();
    let mut pos_to_n = HashMap::new();
    for (i, p) in positions.iter().enumerate() {
        pos_to_n.insert(p, i);
    }

    let mut st_l = SegTree::new(n, 0, |a, b| *a.max(b));
    let mut st_r = SegTree::new(n, l, |a, b| *a.min(b));
    for (c, x) in cxq {
        let i = *pos_to_n.get(&x).unwrap();
        if c == 1 {
            st_l.update(i, x);
            st_r.update(i, x);
        } else {
            let l = st_l.fold(0, i);
            let r = st_r.fold(i, n);
            println!("{}", r - l);
        }
    }
}
