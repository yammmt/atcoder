use proconio::fastout;
use proconio::input;
use proconio::marker::Usize1;

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

#[allow(dead_code)]
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
        for (i, a) in array.iter().enumerate() {
            self.node[i + self.size] = a.clone();
        }
        // 0 はダミーノードだから回さなくてよい
        // ノード i の左を 2i, 右を 2i+1 で表すために好都合
        for i in (1..self.size).rev() {
            // {左,右} の子
            self.node[i] = (self.combine_f)(&self.node[i << 1], &self.node[i << 1 | 1]);
        }
    }

    /// [一点更新] 位置を値で更新
    fn update(&mut self, index: usize, value: T) {
        let mut i = self.size + index;
        self.node[i] = value;
        while i > 1 {
            i >>= 1;
            self.node[i] = (self.combine_f)(&self.node[i << 1], &self.node[i << 1 | 1]);
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
        lrn: [(usize, usize); n - 1],
        abq: [(usize, Usize1); q],
    }

    // インデックスの付け方がややこしくて苦手
    // city: 0, 1, 2, 3, ..., n-1, n
    // l:    0, 1, 2, ..., n-1
    // r:    0, 1, 2, ..., n-1
    // l[i] は city[i] と city[i+1] を結ぶ

    let ln = lrn.iter().map(|&a| a.0).collect::<Vec<usize>>();
    let rn = lrn.iter().map(|&a| a.1).collect::<Vec<usize>>();

    // l 歳以上なら通行可能
    let mut st_l = SegTree::new(n - 1, 0, |&a, &b| a.max(b));
    // r 歳以下なら通行可能
    let mut st_r = SegTree::new(n - 1, usize::MAX, |&a, &b| a.min(b));
    st_l.build(&ln);
    st_r.build(&rn);

    for (a, b) in abq {
        // 始点からいくつ道路を通れるか
        // a 歳の人が都市 b から移動する

        // 数字大方向
        let mut pass_l = b;
        let mut fail = n;
        while fail - pass_l > 1 {
            let mid = (pass_l + fail) / 2;
            if st_l.fold(b, mid) <= a {
                pass_l = mid;
            } else {
                fail = mid;
            }
        }
        let mut pass_r = b;
        let mut fail = n;
        while fail - pass_r > 1 {
            let mid = (pass_r + fail) / 2;
            if st_r.fold(b, mid) >= a {
                pass_r = mid;
            } else {
                fail = mid;
            }
        }
        let ans_upper = pass_l.min(pass_r);

        // 数字小方向
        let mut pass_l = b as isize;
        let mut fail = -1;
        while pass_l - fail > 1 {
            let mid = ((pass_l + fail) / 2) as usize;
            if st_l.fold(mid, b) <= a {
                pass_l = mid as isize;
            } else {
                fail = mid as isize;
            }
        }
        let mut pass_r = b as isize;
        let mut fail = -1;
        while pass_r - fail > 1 {
            let mid = ((pass_r + fail) / 2) as usize;
            if st_r.fold(mid, b) >= a {
                pass_r = mid as isize;
            } else {
                fail = mid as isize;
            }
        }
        let ans_lower = pass_l.max(pass_r) as usize;

        println!("{}", ans_upper - ans_lower + 1);
    }
}
