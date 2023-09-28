use proconio::input;

fn main() {
    input! {
        n: usize,
        an: [i64; n],
    }

    if an.iter().all(|a| *a != 1) {
        println!("-1");
        return;
    }

    let mut ans = 0;
    let mut cur_i = 1;
    for a in &an {
        if *a == cur_i {
            cur_i += 1;
        } else {
            ans += 1;
        }
    }

    println!("{}", ans);
}
