use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        abm: [(usize, usize); m],
        k: usize,
        cdk: [(usize, usize); k],
    }

    let mut ans = 0;
    for i in 0..2u32.pow(k as u32) {
        let mut has_ball = vec![false; n];
        let mut ii = i;
        for cd in &cdk {
            if ii % 2 == 0 {
                has_ball[cd.0 - 1] = true;
            } else{
                has_ball[cd.1 - 1] = true;
            }
            ii /= 2;
        }

        let mut cur = 0;
        for ab in &abm {
            if has_ball[ab.0 - 1] && has_ball[ab.1 - 1] {
                cur += 1;
            }
        }
        ans = ans.max(cur);
    }

    println!("{}", ans);
}
