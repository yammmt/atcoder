use proconio::input;

fn main() {
    input! {
        n: usize,
        mut cn: [u64; n],
    }
    let d = 1_000_000_007;
    cn.sort_unstable();

    let mut ans = 1;
    for (i, c) in cn.iter().enumerate() {
        if i as u64 >= *c {
            ans = 0;
            break;
        }

        ans = (ans * (*c - i as u64)) % d;
    }

    println!("{}", ans);
}
