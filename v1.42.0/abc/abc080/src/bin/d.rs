// :fu: :fu:
// 録画時間 -0.5 分は無視しても通るが嘘解法？

// 1 チャンネルに録画機は最大一台であることをいもす計算五に弾かないと
// [1, 2), [2, 3) で落ちる
// いもすをそのまま足すような別解はない？

use proconio::input;

fn main() {
    input! {
        n: usize,
        c: usize,
        stcn: [(u64, u64, u64); n],
    }

    let mut imos = vec![vec![0; 200004]; c];
    for stc in &stcn {
        imos[stc.2 as usize - 1][2 * stc.0 as usize - 1] += 1;
        imos[stc.2 as usize - 1][2 * stc.1 as usize] -= 1;
    }
    let mut imossum = Vec::with_capacity(c);
    for im in &imos {
        let mut vcur = vec![0; 200004];
        let mut cur = 0;
        for i in 1..200004 {
            vcur[i] = im[i] + cur;
            cur = vcur[i];
        }
        imossum.push(vcur);
    }
    println!("{:?}", imossum);

    let mut ans = 0;
    for i in 0..200004 {
        let mut cur = 0;
        for j in 0..c {
            if imossum[j][i] > 0 {
                cur += 1;
            }
        }
        ans = ans.max(cur);
    }

    println!("{}", ans);
}
