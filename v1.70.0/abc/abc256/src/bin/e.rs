use ac_library::SccGraph;
use proconio::fastout;
use proconio::input;
use proconio::marker::Usize1;

#[fastout]
fn main() {
    input! {
        n: usize,
        xn: [Usize1; n],
        cn: [usize; n],
    }

    // ループがあると誰かしらの不満度が発生する
    // 単純な円状のループであれば不満度最小の者を犠牲者とすればよい
    // ループとは強連結成分分解？
    // 実装だるいのでなにか賢いやりようがありそうだけれど

    let mut graph = SccGraph::new(n);
    for (i, x) in xn.iter().enumerate() {
        graph.add_edge(i, *x);
    }

    let mut ans = 0;
    for g in graph.scc() {
        let mut v = vec![];
        for gg in g {
            v.push(cn[gg]);
        }
        if v.len() > 1 {
            ans += v.iter().min().unwrap();
        }
    }

    println!("{ans}");
}
