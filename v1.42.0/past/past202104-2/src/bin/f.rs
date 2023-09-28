use proconio::input;

fn main() {
    input! {
        n: usize,
        l: usize,
        t: usize,
        x: usize,
        abn: [(usize, usize); n],
    }

    if abn.iter().any(|&ab| ab.1 >= l && ab.0 > t) {
        println!("forever");
        return;
    }

    let mut cur_time = 0;
    let mut ans = 0;
    for ab in &abn {
        if ab.1 < l {
            cur_time = 0;
            ans += ab.0;
        } else {
            if cur_time + ab.0 > t {
                ans += t - cur_time;
                ans += x;
                cur_time = 0;
            }

            ans += ab.0;
            if cur_time + ab.0 == t {
                ans += x;
                cur_time = 0;
            } else {
                cur_time += ab.0;
            }
        }
    }

    println!("{}", ans);
}
