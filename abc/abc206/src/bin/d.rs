// WA: an の範囲でバカアホドジマヌケした

use petgraph::unionfind::UnionFind;
use proconio::input;

static ANMAX: usize = 2 * 100_000 + 3;

fn main() {
    input! {
        n: usize,
        an: [usize; n],
    }

    // 同じ数字を違う数字に変換しなくてはならない場合が明らかにある
    // 有効グラフを作って辺の本数, ループができたらそこから -1?
    // むしろ有向グラフ内の要素数 - 1 で十分では？
    let mut uf = UnionFind::new(ANMAX);
    for i in 0..n / 2 {
        uf.union(an[i], an[n - i - 1]);
    }
    let mut grpnum = vec![0; ANMAX];
    for i in 0..ANMAX {
        grpnum[uf.find(i)] += 1;
    }
    // このアルゴリズムでは明らかに i32 の範疇を超えない
    let mut ans = 0usize;
    for i in 0..ANMAX {
        if grpnum[i] > 1 {
            ans += grpnum[i] - 1;
        }
    }

    println!("{}", ans);
}
