// 24.5min (WA: 22min)
// WA: 数え上げが i32 型 何度目だこれ

// 重実装？書き方が悪いだけ？

use petgraph::unionfind::UnionFind;
use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        s1: Chars,
        s2: Chars,
    }

    // とりあえず変換テーブルを作る
    let mut a2n = vec![None; 26];
    for i in 0..n {
        let c1_upper = s1[i].is_ascii_uppercase();
        let c2_upper = s2[i].is_ascii_uppercase();
        if c1_upper && !c2_upper {
            a2n[(s1[i] as u8 - b'A') as usize] = Some(s2[i]);
        } else if !c1_upper && c2_upper {
            a2n[(s2[i] as u8 - b'A') as usize] = Some(s1[i]);
        }
    }

    // 異なる種類のアルファベットの基の数字が同じ数字である場合
    // 1A23
    // AB23
    let mut uf = UnionFind::new(26);
    for i in 0..n {
        let c1_upper = s1[i].is_ascii_uppercase();
        let c2_upper = s2[i].is_ascii_uppercase();
        if c1_upper && c2_upper {
            uf.union((s1[i] as u8 - b'A') as usize, (s2[i] as u8 - b'A') as usize);
        }
    }
    let mut abc_grp = vec![0; 26];
    for i in 0..26 {
        abc_grp[i] = uf.find(i);
    }
    let mut abc2n = vec![None; 26];
    for i in 0..26 {
        if let Some(cur) = a2n[i] {
            abc2n[abc_grp[i]] = Some(cur);
        }
    }

    // 出現可否
    let mut is_exist = vec![false; 26];
    for i in 0..n {
        let c1_upper = s1[i].is_ascii_uppercase();
        let c2_upper = s2[i].is_ascii_uppercase();
        if c1_upper {
            is_exist[(s1[i] as u8 - b'A') as usize] = true;
        }
        if c2_upper {
            is_exist[(s2[i] as u8 - b'A') as usize] = true;
        }
    }

    let mut ans = 1usize;
    for i in 0..26 {
        if !is_exist[i] {
            continue;
        }

        if abc2n[abc_grp[i]].is_none() {
            if b'A' + i as u8 == s1[0] as u8 || b'A' + i as u8 == s2[0] as u8 {
                // 先頭に 0 は不可
                ans *= 9;
            } else {
                ans *= 10;
            }
        }
        // 辻褄
        abc2n[abc_grp[i]] = Some('0');
    }

    println!("{}", ans);
}
