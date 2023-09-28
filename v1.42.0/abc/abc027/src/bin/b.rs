// 6.5min

use proconio::input;

fn main() {
    input! {
        n: usize,
        an: [i32; n],
    }

    let asum = an.iter().sum::<i32>();
    if asum % n as i32 != 0 {
        println!("-1");
        return;
    }

    let mut ans = 0;
    let needed = asum / n as i32;
    let mut csum = 0;
    let mut streak = 0;
    for a in &an {
        streak += 1;
        csum += *a - needed;
        if csum == 0 {
            ans += streak - 1;
            streak = 0;
        }
    }
    println!("{}", ans);
}
