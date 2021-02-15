// 50min

// :fu: :fu: 数問 サンプルは親切
// 拡張ユークリッドで (も) 解けそう

use proconio::input;

fn main() {
    input! {
        n: usize,
        tan: [(u64, u64); n],
    }

    let mut ans = (1, 1);
    for ta in &tan {
        let n = ((ans.0 + ta.0 - 1) / ta.0).max((ans.1 + ta.1 - 1) / ta.1);
        ans = (ta.0 * n, ta.1 * n);
    }

    println!("{}", ans.0 + ans.1);
}
