// 数字が大きく単純なシミュレーションでは TLE
// 貪欲は不可
// 倍にするタイミングがある程度増やした/減らした後の場合があり...
// どこかで似た問題を見たことがあるようなないような

use proconio::input;

fn main() {
    input! {
        x: i64,
        y: i64,
    }

    if x == y {
        println!("0");
        return;
    }

    let mut ans = (x - y).abs();

    // esp
    let xymax = x.max(y);
    let xymin = x.min(y);
    let mut xnum = 1;
    let mut xcur = xymin;
    while xcur <= xymax {
        xcur *= 2;
        xnum += 1;
    }
    xnum -= 1;
    // println!("xcur: {}", xcur);
    // println!("xnum: {}", xnum);
    let from_up = xcur - xymax;
    ans = ans.min(from_up + xnum);
    xcur /= 2;
    let from_down = xymax - xcur;
    ans = ans.min(from_down + xnum - 1);

    println!(
        "{}",
        ans
    );
}
