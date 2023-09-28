// 16min

use proconio::input;

fn bit_rows(n: u32) -> Vec<Vec<usize>> {
    let mut ret = vec![];
    for b in 0..2u64.pow(n) {
        let mut cur = vec![];
        for i in 0..n {
            if b & (1 << i) > 0 {
                cur.push(i as usize);
            }
        }
        ret.push(cur);
    }
    ret
}

fn main() {
    input! {
        d: usize,
        g: usize,
        pcd: [(usize, usize); d],
    }

    let mut ipcd = vec![];
    for (i, p) in pcd.iter().enumerate() {
        ipcd.push((i, p.0, p.1));
    }

    let mut ans = std::usize::MAX / 2;
    // コンプリートボーナスを得る点数帯を決め打ちして不足分は高得点のものから順に解いていって補う
    let completed_rows = bit_rows(d as u32);
    for cmp_rows in &completed_rows {
        let mut pts = 0;
        let mut solved_num = vec![0; d];
        for c in cmp_rows {
            solved_num[ipcd[*c].0] = ipcd[*c].1;
            pts += (ipcd[*c].0 + 1) * 100 * ipcd[*c].1 + ipcd[*c].2;
        }

        while pts < g {
            for i in (0..d).rev() {
                if solved_num[i] < ipcd[i].1 {
                    solved_num[i] += 1;
                    pts += (ipcd[i].0 + 1) * 100;
                    if solved_num[i] == ipcd[i].1 {
                        pts += ipcd[i].2;
                    }
                    break;
                }
            }
        }

        ans = ans.min(solved_num.iter().sum::<usize>());
    }

    println!("{}", ans);
}
