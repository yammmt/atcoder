// :fu: :fu: :fu: :fu: :fu: 21-06 少なくとも個人的には最悪級で模範解答なぞっても WA
// f32 型ではこの桁数ですら計算誤差で WA
// 誤読除いて令和 ABC 最難では Ryuma より辛かった

use proconio::input;

fn to_closed(tlr: (usize, f64, f64)) -> (f64, f64) {
    match tlr.0 {
        1 => (tlr.1, tlr.2),
        2 => (tlr.1, tlr.2 - 0.5),
        3 => (tlr.1 + 0.5, tlr.2),
        4 => (tlr.1 + 0.5, tlr.2 - 0.5),
        _ => unreachable!(),
    }
}

fn main() {
    input! {
        n: usize,
        tlrn: [(usize, f64, f64); n],
    }

    let mut ans = 0;
    for i in 0..n {
        let i_area = to_closed(tlrn[i]);
        // println!("i: {:?}", i_area);
        for j in i + 1..n {
            let j_area = to_closed(tlrn[j]);
            // println!("  j: {:?}", j_area);
            if i_area.0.max(j_area.0) <= i_area.1.min(j_area.1) {
                ans += 1;
            }
        }
    }

    println!("{}", ans);
}
