use proconio::input;

fn main() {
    input! {
        n: usize,
        an: [i64; n],
    }

    let mut ans = n as i64;
    for i in 0..n {
        let mut curmin = std::i64::MAX;
        for (j, a) in an.iter().skip(i).enumerate() {
            curmin = curmin.min(*a);
            ans = ans.max(curmin * (j as i64 + 1));
        }
    }

    println!("{}", ans);
}
