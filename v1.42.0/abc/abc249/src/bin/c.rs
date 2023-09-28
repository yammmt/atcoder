use proconio::input;
use proconio::marker::Bytes;

fn main() {
    input! {
        n: usize,
        k: usize,
        sn: [Bytes; n],
    }

    let mut ans = 0;
    for i in 0..2u32.pow(n as u32) {
        let mut charcnt = vec![0; 26];
        let mut ii = i;
        let mut j = 0;
        while ii > 0 {
            if ii % 2 == 1 {
                for b in &sn[j] {
                    charcnt[(*b - b'a') as usize] += 1;
                }
            }

            ii /= 2;
            j += 1;
        }

        let cur = charcnt.iter().filter(|&c| *c == k).count();
        ans = ans.max(cur);
    }

    println!("{}", ans);
}
