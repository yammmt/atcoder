use proconio::input;

fn main() {
    input! {
        n: usize,
        t: u64,
        ctn: [(u64, u64); n],
    }

    let mut ans = std::u64::MAX;
    for ct in &ctn {
        if ct.1 > t {
            continue;
        }

        ans = ans.min(ct.0);
    }

    if ans == std::u64::MAX {
        println!("TLE");
    } else {
        println!("{}", ans);
    }
}
