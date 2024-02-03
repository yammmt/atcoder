use petgraph::unionfind::UnionFind;
use proconio::fastout;
use proconio::input;
use proconio::marker::Usize1;

#[fastout]
fn main() {
    input! {
        n: usize,
        an: [Usize1; n],
    }
    let a_max = an.iter().max().unwrap() + 1;

    let mut uf = UnionFind::new(a_max);
    for i in 0..=n / 2 {
        uf.union(an[i], an[n - i - 1]);
    }

    let mut member_num = vec![0; a_max];
    let mut an_dedup = an.clone();
    an_dedup.sort_unstable();
    an_dedup.dedup();
    for a in an_dedup {
        member_num[uf.find(a)] += 1;
    }

    let mut ans = 0;
    for m in member_num {
        if m == 0 {
            continue;
        }

        ans += m - 1;
    }

    println!("{ans}");
}
