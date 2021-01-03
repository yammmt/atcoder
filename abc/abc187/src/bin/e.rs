// TODO: ごちゃごちゃしているので模範解答見て書き改める

use proconio::input;
use std::collections::VecDeque;

fn main() {
    input! {
        n: usize,
        abn_old: [(usize, usize); n - 1],
        q: usize,
        texq: [(usize, usize, i64); q],
    }
    let mut abn = vec![(0, 0)];
    for ab in &abn_old {
        abn.push(*ab);
    }

    let mut edges = vec![vec![]; n + 1];
    for ab in &abn {
        edges[ab.0].push(ab.1);
        edges[ab.1].push(ab.0);
    }

    let mut vdq = VecDeque::new();
    vdq.push_back(1);
    let mut iki_idx = vec![0; n + 1];
    let mut kaeri_idx = vec![0; n + 1];
    let mut imos_idx = vec![0; 2 * n + 1];
    let mut idx = 0;
    while let Some(cur) = vdq.pop_back() {
        // println!("cur: {}", cur);
        idx += 1;
        imos_idx[idx] = cur;
        if iki_idx[cur] == 0 {
            iki_idx[cur] = idx;
        } else {
            kaeri_idx[cur] = idx;
        }
        if kaeri_idx[cur] != 0 {
            continue;
        }

        vdq.push_back(cur);
        for e in &edges[cur] {
            if iki_idx[*e] == 0 {
                vdq.push_back(*e);
            }
        }
    }
    // println!("iki: {:?}", iki_idx);
    // println!("kaeri: {:?}", kaeri_idx);
    // println!("imos_idx: {:?}", imos_idx);

    let mut imos = vec![0; 2 * n + 1];
    for tex in &texq {
        // println!("{:?}", tex);
        let mut start_pt = abn[tex.1].0;
        let mut ng_pt = abn[tex.1].1;
        if tex.0 == 2 {
            std::mem::swap(&mut start_pt, &mut ng_pt);
        }

        if iki_idx[start_pt] < iki_idx[ng_pt] {
            // NG 点を根とする木が始点を根とする木に含まれる
            imos[0] += tex.2;
            imos[iki_idx[ng_pt]] -= tex.2;
            imos[kaeri_idx[ng_pt] + 1] += tex.2;
        } else {
            // NG 点を根とする木が始点を根とする木を含む
            imos[iki_idx[start_pt]] += tex.2;
            imos[kaeri_idx[start_pt] + 1] -= tex.2;
        }
    }
    // println!("imos: {:?}", imos);

    // calc imos
    let mut imossum = vec![0; 2 * n + 1];
    imossum[0] = imos[0];
    for i in 1..imos.len() {
        imossum[i] = imossum[i - 1] + imos[i];
    }
    // println!("imossum: {:?}", imossum);

    let mut ans = vec![0; n + 1];
    for i in 1..imossum.len() {
        ans[imos_idx[i]] += imossum[i];
    }

    for a in ans.iter().skip(1) {
        // 同頂点を二度計算している都合
        println!("{}", a / 2);
    }
}
