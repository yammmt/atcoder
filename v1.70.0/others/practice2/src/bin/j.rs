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
        an: [usize; n],
    }

    let mut st = SegTree::new(n, 0, |&a, &b| a.max(b));
    st.build(&an);

    for _ in 0..q {
        input! {
            t: usize,
        }
        match t {
            1 => {
                input! {
                    x: Usize1,
                    v: usize,
                }
                st.update(x, v);
            }
            2 => {
                // [l, r] を問われているため, r を Usize1 にすると狭まる
                input! {
                    l: Usize1,
                    r: usize,
                }
                println!("{}", st.fold(l, r));
            }
            3 => {
                input! {
                    x: Usize1,
                    v: usize,
                }
                let mut pass = n as isize;
                let mut fail = x as isize - 1;
                while pass - fail > 1 {
                    let mid = (pass + fail) / 2;
                    let midu = mid as usize;
                    // mid に含まれる場合も pass を更新する (閉区間)
                    if st.fold(x, midu + 1) >= v {
                        pass = mid;
                    } else {
                        fail = mid;
                    }
                }
                println!("{}", pass + 1);
            }
            _ => unreachable!(),
        }
    }
}
