use petgraph::unionfind::UnionFind;
use proconio::fastout;
use proconio::input;
use proconio::marker::Usize1;

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
    }

    // コストが小さい順に貪欲につなげていけばよい
    // sum(k_i) がそれほど大きくないから, 連結を二乗オーダーでやらなければなんとかなりそう

    let mut cm = vec![];
    let mut am = vec![];

    for i in 0..m {
        input! {
            k: usize,
            c: usize,
            ak: [Usize1; k],
        }
        cm.push((c, i));
        am.push(ak);
    }
    cm.sort_unstable();

    let mut ans = 0;
    let mut uf = UnionFind::new(n);
    for (c, i) in cm {
        for j in 1..am[i].len() {
            let group_a = uf.find(am[i][j - 1]);
            let group_b = uf.find(am[i][j]);
            if group_a == group_b {
                continue;
            }

            uf.union(am[i][j - 1], am[i][j]);
            ans += c;
        }
    }

    let mut member_num = vec![0; n];
    for i in 0..n {
        member_num[uf.find(i)] += 1;
    }

    if member_num.iter().any(|&m| m == n) {
        println!("{ans}");
    } else {
        println!("-1");
    }
}
