use proconio::input;

fn main() {
    input! {
        n: usize,
        abn: [(i64, i64); n],
    }

    let mut aoki = 0;
    let mut base_pts = vec![];
    for ab in &abn {
        aoki += ab.0;
        base_pts.push((2 * ab.0 + ab.1, ab.0, ab.1));
    }
    base_pts.sort_unstable();
    base_pts.reverse();

    let mut ans = 0;
    let mut takahashi = 0;
    for pts in &base_pts {
        takahashi += pts.1 + pts.2;
        aoki -= pts.1;
        // println!("a: {}", aoki);
        // println!("t: {}", takahashi);
        ans += 1;
        if takahashi > aoki {
            break;
        }
    }
    println!("{}", ans);
}
