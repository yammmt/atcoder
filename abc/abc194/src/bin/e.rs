// 遅い

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        an: [usize; n],
    }

    let mut ans = n + 1;

    let mut cnt = vec![0; n + 2];
    for i in 0..m {
        cnt[an[i]] += 1;
    }
    for (i, c) in cnt.iter().take(n + 2).enumerate() {
        if *c == 0 {
            ans = i;
            break;
        }
    }
    // println!("{:?}", cnt);

    for i in m..n {
        // println!("to: {}", i);
        let removed_i = i - m;
        cnt[an[i]] += 1;
        cnt[an[removed_i]] -= 1;
        if cnt[an[removed_i]] == 0 {
            ans = ans.min(an[removed_i]);
        }
        // println!("{:?}", cnt);
    }

    println!("{}", ans);
}
