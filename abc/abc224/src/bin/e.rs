use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        n: usize,
        rcan: [(usize, usize, usize); n],
    }

    // 自身に対して移動してくることのできる頂点にグラフ貼ってトポロジカルソートするとグラフ作成時点で O(N^2)

    // 自然数降順 (探索順)
    let mut arcin = vec![];
    for (i, rca) in rcan.iter().enumerate() {
        arcin.push((rca.2, rca.0, rca.1, i));
    }
    arcin.sort_unstable();
    arcin.reverse();

    let mut ans = vec![0; n];
    let mut amax_h = vec![0; h];
    let mut amax_w = vec![0; w];
    let mut updated_arci = vec![];
    for (i, arci) in arcin.iter().enumerate() {
        ans[arci.3] = amax_h[arci.1 - 1].max(amax_w[arci.2 - 1]);
        updated_arci.push(*arci);

        if i == n - 1 || arcin[i + 1].0 != arci.0 {
            for x in &updated_arci {
                amax_h[x.1 - 1] = amax_h[x.1 - 1].max(ans[x.3] + 1);
                amax_w[x.2 - 1] = amax_w[x.2 - 1].max(ans[x.3] + 1);
            }
            updated_arci.clear();
        }
    }

    for a in &ans {
        println!("{}", a);
    }
}
