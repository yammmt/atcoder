use proconio::fastout;
use proconio::input;

#[allow(dead_code)]
#[derive(Debug)]
struct LazySegTree<T1, T2, F1, F2, F3>
where
    T1: Clone + std::cmp::PartialEq,
    T2: Clone + std::cmp::PartialEq,
    F1: Fn(&T1, &T1) -> T1,
    F2: Fn(&T2, &T2) -> T2,
    F3: Fn(&T1, &T2) -> T1,
{
    n: usize,
    size: usize,
    height: usize,
    node: Vec<T1>,
    lazy: Vec<T2>,
    /// 値データの単位元
    identity_e_node: T1,
    /// 遅延データの単位元
    identity_e_lazy: T2,
    /// 値データの値を合成するための関数
    combine_f_node: F1,
    /// 遅延データの値を合成するための関数
    combine_f_lazy: F2,
    /// 遅延データを値データに反映させるための関数
    reflect_f: F3,
}

#[allow(dead_code)]
impl<T1, T2, F1, F2, F3> LazySegTree<T1, T2, F1, F2, F3>
where
    T1: Clone + std::cmp::PartialEq,
    T2: Clone + std::cmp::PartialEq,
    F1: Fn(&T1, &T1) -> T1,
    F2: Fn(&T2, &T2) -> T2,
    F3: Fn(&T1, &T2) -> T1,
{
    fn new(
        n: usize,
        identity_e_node: T1,
        identity_e_lazy: T2,
        combine_f_node: F1,
        combine_f_lazy: F2,
        reflect_f: F3,
    ) -> Self {
        let mut size = 1;
        let mut height = 0;
        while size < n {
            size *= 2;
            height += 1;
        }
        let node = vec![identity_e_node.clone(); 2 * size];
        let lazy = vec![identity_e_lazy.clone(); 2 * size];

        LazySegTree {
            n,
            size,
            height,
            node,
            lazy,
            identity_e_node,
            identity_e_lazy,
            combine_f_node,
            combine_f_lazy,
            reflect_f,
        }
    }

    /// 遅延データの値を値データに反映させたときの結果を返す
    fn _reflect_lazy(&self, index: usize) -> T1 {
        (self.reflect_f)(&self.node[index], &self.lazy[index])
    }

    /// [遅延評価] `index` 番目 (0-indexed) の要素を含む区間について, 遅延データを伝搬させる.
    /// 根に近いものから処理される
    fn _propagate_from_top(&mut self, index: usize) {
        let index = index + self.size;
        // h は右シフト量だから, 大きいほどインデックスが小さくなり根に近付く
        // => "根に近いものから処理される"
        for h in (1..=self.height).rev() {
            let i = index >> h;
            if self.lazy[i] != self.identity_e_lazy {
                // if self.lazy[i] != self.identity_e_lazy {
                // 遅延データの情報を子に伝搬させる
                self.lazy[i << 1] = (self.combine_f_lazy)(&self.lazy[i << 1], &self.lazy[i]);
                self.lazy[i << 1 | 1] =
                    (self.combine_f_lazy)(&self.lazy[i << 1 | 1], &self.lazy[i]);

                // 遅延データの情報を値データに反映させ, 遅延データの値をリセット
                self.node[i] = self._reflect_lazy(i).clone();
                self.lazy[i] = self.identity_e_lazy.clone();
            }
        }
    }

    /// `index` 番目 (0-indexed) の要素を表す葉から順に値データを確定させる
    /// 正確には, 葉に対しては行っておらず, 葉の親から順に確定させている
    fn _update_from_bottom(&mut self, index: usize) {
        let mut index = (index + self.size) >> 1;
        while index > 0 {
            self.node[index] = (self.combine_f_node)(
                &self._reflect_lazy(index << 1),
                &self._reflect_lazy(index << 1 | 1),
            );
            index >>= 1;
        }
    }

    /// 配列の各要素を登録する
    fn build(&mut self, array: &[T1]) {
        assert_eq!(array.len(), self.n);
        for (i, a) in array.iter().enumerate() {
            self.node[i + self.size] = a.clone();
        }
        for i in (1..self.size).rev() {
            self.node[i] = (self.combine_f_node)(&self.node[i << 1], &self.node[i << 1 | 1])
        }
    }

    /// [区間更新] 位置 [L, R) (0-indexed) を値 `value` で更新する
    fn update(&mut self, l: usize, r: usize, value: T2) {
        // トップダウンに遅延データの値を子に伝搬させる
        self._propagate_from_top(l);
        self._propagate_from_top(r - 1);

        // 入力に対応する区間のついて遅延データを更新
        let mut l_lazy = l + self.size;
        let mut r_lazy = r + self.size;
        while l_lazy < r_lazy {
            if l_lazy % 2 == 1 {
                self.lazy[l_lazy] = (self.combine_f_lazy)(&self.lazy[l_lazy], &value);
                l_lazy += 1;
            }
            if r_lazy % 2 == 1 {
                r_lazy -= 1;
                self.lazy[r_lazy] = (self.combine_f_lazy)(&self.lazy[r_lazy], &value);
            }
            l_lazy >>= 1;
            r_lazy >>= 1;
        }
        self._update_from_bottom(l);
        self._update_from_bottom(r - 1);
    }

    /// [区間取得] [l, r) (0-indexed) 内の要素について, `l` 番目から順に
    /// `combine_f_node` を適用した結果を返す (交換法則が前提になくともよい)
    fn fold(&mut self, l: usize, r: usize) -> T1 {
        // トップダウンに遅延データの値を子に伝搬させる
        self._propagate_from_top(l);
        self._propagate_from_top(r - 1);

        // 入力に対応する区間について値を取得して合成
        let mut l = l + self.size;
        let mut r = r + self.size;
        let mut value_l = self.identity_e_node.clone();
        let mut value_r = self.identity_e_node.clone();
        while l < r {
            if l % 2 == 1 {
                value_l = (self.combine_f_node)(&value_l, &self._reflect_lazy(l)).clone();
                l += 1;
            }
            if r % 2 == 1 {
                r -= 1;
                value_r = (self.combine_f_node)(&self._reflect_lazy(r), &value_r).clone();
            }
            l >>= 1;
            r >>= 1;
        }
        (self.combine_f_node)(&value_l, &value_r)
    }
}

#[fastout]
fn main() {
    const DUMMY: usize = usize::MAX / 2;
    const MOD: usize = 998_244_353;

    input! {
        n: usize,
        q: usize,
        lrdq: [(usize, usize, usize); q],
    }

    // - 値データとして, (文字列を整数と見做したときの値, 桁数) のペアを管理する
    // - 値データの合成は, 左側の子が持つ値を, 右側の子が持つ桁数ぶんだけずらして足す
    // - 遅延データとして, 区間全体に対して更新があった数字 d を管理する
    // - 値データへの反映は, 値データが管理する, 文字列を整数と見做したときの値を,
    //   桁数ぶんだけ d を並べた数で更新することによって行える
    //   - これがわからん...

    let mut ten = Vec::with_capacity(n + 1);
    let mut one = Vec::with_capacity(n + 1);
    ten.push(1);
    one.push(0);
    for i in 1..n + 1 {
        ten.push((ten[i - 1] * 10) % MOD);
        one.push((one[i - 1] + ten[i - 1]) % MOD);
    }

    let combine_node = |a: &(usize, usize), b: &(usize, usize)| ((a.0 * ten[b.1] + b.0) % MOD, a.1 + b.1);
    let combine_lazy = |_a: &usize, b: &usize| *b;
    let reflect = |node: &(usize, usize), lazy: &usize| {
        if *lazy == DUMMY {
            *node
        } else {
            (one[node.1] * lazy % MOD, node.1)
        }
    };

    let mut lst = LazySegTree::new(n, (0, 0), DUMMY, combine_node, combine_lazy, reflect);

    let a = vec![(1, 1); n];
    lst.build(&a);

    for (l, r, d) in lrdq {
        let l = l - 1;
        lst.update(l, r, d);
        println!("{}", lst.fold(0, n).0);
    }
}
