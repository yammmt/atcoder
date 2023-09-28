use proconio::input;

fn main() {
    input! {
        n: usize,
        mut an: [i64; n],
        mut bn: [i64; n],
        mut cn: [i64; n],
    }
    an.sort_unstable();
    bn.sort_unstable();
    cn.sort_unstable();

    let mut ans = 0;
    let mut b_idx = 0;
    let mut c_idx = 0;
    for a in &an {
        while b_idx < n && bn[b_idx] <= *a {
            b_idx += 1;
        }
        if b_idx >= n {
            break;
        }

        while c_idx < n && cn[c_idx] <= bn[b_idx] {
            c_idx += 1;
        }
        if c_idx >= n {
            break;
        }

        ans += 1;
        b_idx += 1;
        c_idx += 1;
    }

    println!("{}", ans);
}
