use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
    }

    let mut ans = 0;
    for i in 0..=9999 {
        let mut cur_uses = vec![false; 10];
        let mut ii = i;
        for _ in 0..4 {
            cur_uses[ii % 10] = true;
            ii /= 10;
        }

        let mut cur_passes = true;
        for (i, c) in s.iter().enumerate() {
            match *c {
                'o' => {
                    if !cur_uses[i] {
                        cur_passes = false;
                        break;
                    }
                }
                'x' => {
                    if cur_uses[i] {
                        cur_passes = false;
                        break;
                    }
                }
                _ => {}
            }
        }

        if cur_passes {
            ans += 1;
        }
    }

    println!("{ans}");
}
