use proconio::fastout;
use proconio::input;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn upper_bound(v: usize, arr: &[(usize, isize)]) -> usize {
    let mut pass = 0;
    let mut fail = arr.len();
    while fail - pass > 1 {
        let mid = (pass + fail) / 2;
        if arr[mid].0 <= v {
            pass = mid;
        } else {
            fail = mid;
        }
    }
    pass
}

#[fastout]
fn main() {
    input! {
        n: usize,
        w: usize,
        c: usize,
        mut lrpn: [(usize, usize, isize); n],
    }
    lrpn.sort_unstable();
    // 石が占拠する区間は開区間？そうじゃないと例 1 が説明できない
    // 問題文の書き方が悪い...
    // 閉区間にする
    let mut v = vec![];
    for (l, r, p) in lrpn {
        v.push((l + 1, r - 1, p));
    }
    let lrpn = v;

    // 取り除き始める左端を決めたら右端は区間長 c に達するまで全部除去する必要がある
    // 第一印象は尺取りとか二分探索の類
    // 区間全体にかかる長さの石を考えると、到達時点での累積撤去費用だけでは不十分で
    // イベントソートと累積和との併用が必要そう

    // イベントソートして区間開始時のコストを取る
    // + 開始時基準の累積和で区間中に取り始める分のコストを取る
    // 始点は石置かれているところ全て, 対応する位置は二分探索で O(NlogN)
    // 石を取り除く必要がない場合を拾える？区間左端右端の場合に探索させるでも不十分そう
    // 石を取り除き終わった次の位置を始点に探索させる, でなんとなくうまくいきそうだけど
    // 探索始点も, 石始点/終点/終点の次としても 3e5 通りしかない
    // そもそもどうせ取らなくちゃならないなら, 石区間の終了間際に取る戦法は
    // 区間途中に石区間がある分をスルーできる利点がある

    // 取り除ける区間最長となる開始地点と, 取り除く必要性のなくなる終了次の地点を全探索,
    // で直感的にはよいとは思うのだけれど

    // 開始地点時点で発生しているコスト
    // 区間幅 c より長い石があることを踏まえると必要
    let mut heap = BinaryHeap::new();
    for &(l, r, p) in &lrpn {
        heap.push(Reverse((l, p)));
        heap.push(Reverse((r + 1, -p)));
    }
    // 番兵
    heap.push(Reverse((w + 1, 0)));
    let mut cost_begin = vec![(0, 0)];
    while let Some(Reverse((pos_cur, cost_diff))) = heap.pop() {
        let cost_last = cost_begin.last().unwrap().1;
        cost_begin.push((pos_cur, cost_last + cost_diff));
    }
    // println!("cost_begin: {:?}", cost_begin);

    // [0, 開始地点] の区間を開通させるに必要なコスト
    // 開始地点が同じ点が交じるが無視してよいはず
    let mut cusum_begin = vec![(0, 0)];
    for &(l, _r, p) in &lrpn {
        let cost_last = cusum_begin.last().unwrap().1;
        cusum_begin.push((l, cost_last + p));
    }
    // 番兵
    cusum_begin.push((w + 1, cusum_begin.last().unwrap().1));
    // println!("cusum_begin: {:?}", cusum_begin);

    let cost = |c_begin: usize| {
        let mut ret = 0;
        // 始点時点での区間コストと終点までに取り除き始めるコストの和
        // 始点時点ですでに撤去し始めている分のコスト
        let i = upper_bound(c_begin, &cost_begin);
        ret += cost_begin[i].1;

        // 終点時点での撤去コスト総和
        let i = upper_bound(c_begin + c, &cusum_begin);
        ret += cusum_begin[i].1;

        // 始点時点での撤去コスト総和
        let i = upper_bound(c_begin, &cusum_begin);
        ret -= cusum_begin[i].1;

        // println!("  from: {c_begin}, cost: {ret}");
        ret
    };

    let mut ans = isize::MAX / 2;
    // for i in 0..w-c {
    //     ans = ans.min(cost(i));
    // }
    for &(l, r, _p) in &lrpn {
        // 石を区間左右両端とする場合で四通り
        // 取り除く必要のない限界位置とどうせ取り除くなら最も長い区間を掃除できる位置
        for &a in &[l - 1, r + 1] {
            if a >= c {
                // println!("{}", a-c);
                ans = ans.min(cost(a - c));
            }
            if a + c <= w {
                // println!("{}", a);
                ans = ans.min(cost(a));
            }
        }
    }

    println!("{ans}");
}
