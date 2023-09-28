use proconio::input;

fn main() {
    input! {
        n: usize,
        w: usize,
        mut abn: [(usize, usize); n],
    }

    abn.sort_unstable();
    abn.reverse();

    let mut ans = 0;
    let mut cur_w = 0;
    for ab in &abn {
        let used = ab.1.min(w - cur_w);
        ans += ab.0 * used;
        cur_w += used;
    }

    println!("{}", ans);
}
