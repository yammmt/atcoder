// :fu: 21-04 diff の割には解きたい でも DP 遷移がわからないので EDPC もう少し見るべきか
// 二点間しかみないとサンプル 4 で落ちる

use permutohedron::heap_recursive;
use proconio::input;

// v[i] >= k を満たす最小の i を返す (or k 以下である v の要素数)
// 見つからなければ `v.len()` を返す
// `v` はソートされていること
fn lower_bound(v: &[usize], k: usize) -> usize {
    let mut low: isize = -1;
    let mut high = v.len() as isize;

    while high - low > 1 {
        let mid = (low + high) / 2;
        if v[mid as usize] >= k {
            high = mid;
        } else {
            low = mid;
        }
    }
    high as usize
}

fn main() {
    input! {
        n: usize,
        m: usize,
        wn: [usize; n],
        mut lvm: [(usize, usize); m],
    }

    // 一匹ですら渡れなければ不可能
    let mut vmax = 0;
    lvm.iter().for_each(|lv| vmax = vmax.max(lv.1));
    if *wn.iter().max().unwrap() > vmax {
        println!("-1");
        return;
    }

    // すべての重さに対してこれだけ距離を取らなければならないという値を前計算しておく
    // パーツの重さは重複しうるが dedup してもしなくても同じ
    lvm.sort_unstable_by(|a, b| (a.1).cmp(&b.1));
    // println!("{:?}", lvm);
    // まっとうなコードではタプルかなにかで管理すべき
    let mut required_w = vec![];
    let mut required_dist = vec![];
    for (i, lv) in lvm.iter().enumerate() {
        required_w.push(lv.1);
        required_dist.push(
            if i == 0 {
                lv.0
            } else {
                lv.0.max(required_dist[i - 1])
            }
        );
    }

    // ラクダの順番は全通り試す
    let mut permutations = vec![];
    let mut data = (0..n).collect::<Vec<usize>>();
    heap_recursive(&mut data, |p| {
        permutations.push(p.to_vec());
    });

    let mut ans = std::usize::MAX / 2;
    for ps in &permutations {
        // println!("{:?}", ps);
        let mut cur_w = 0;
        let mut cur_d = 0;

        for i in ps {
            let next_w = cur_w + wn[*i];
            // println!("next_w: {}", next_w);
            if next_w <= required_w[0] {
                cur_w = next_w;
                continue;
            }

            let dist_i = lower_bound(&required_w, next_w);
            // println!("dist_i: {}", dist_i);
            cur_d += required_dist[dist_i - 1];
            cur_w = wn[*i];
        }

        // println!("  {}", cur_d);
        ans = ans.min(cur_d);
    }

    println!("{}", ans);
}
