// 35min
// 割り算の位置で + 10min 以上

use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
    }

    let mut ans = h * w;
    // 上から i 行を一つ目の領域
    for i in 1..h {
        // println!("i: {}", i);
        let area1 = i * w;
        let h_left = h - i;
        // 残りを行単位で分割
        let area2 = (h_left + 1) / 2 * w;
        let area3 = h_left / 2 * w;
        // println!("  {} {} {}", area1, area2, area3);
        if area3 != 0  {
            ans = ans.min(
                area1.max(area2.max(area3)) - area1.min(area2.min(area3))
            );
        }
        // 残りを列単位で分割
        let area2 = h_left * ((w + 1) / 2);
        let area3 = h_left * (w / 2);
        if area3 != 0 {
            ans = ans.min(
                area1.max(area2.max(area3)) - area1.min(area2.min(area3))
            );
        }
        // println!("  {} {} {}", area1, area2, area3);
        // println!("  {}", ans);
    }

    // 左から i 行を一つ目の領域
    for i in 1..w {
        // println!("i: {}", i);
        let area1 = i * h;
        let w_left = w - i;
        // 残りを行単位で分割
        let area2 = (h + 1) / 2 * w_left;
        let area3 = h / 2 * w_left;
        if area3 != 0 {
            ans = ans.min(
                area1.max(area2.max(area3)) - area1.min(area2.min(area3))
            );
        }
        // 残りを列単位で分割
        let area2 = h * ((w_left + 1) / 2);
        let area3 = h * (w_left / 2);
        if area3 != 0 {
            ans = ans.min(
                area1.max(area2.max(area3)) - area1.min(area2.min(area3))
            );
        }
        // println!("  {}", ans);
    }

    println!("{}", ans);
}
