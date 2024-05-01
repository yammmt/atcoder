use proconio::fastout;
use proconio::input;
use proconio::marker::Chars;

#[fastout]
fn main() {
    input! {
        n: usize,
        pvn: [(char, Chars); n],
    }

    let mut ans = vec![None; 6];
    for (i, (p, v)) in pvn.iter().enumerate() {
        let ii = (*p as u8 - b'A') as usize;
        let ps = v.iter().collect::<String>();
        if ans[ii].is_none() && &ps == "AC" {
            ans[ii] = Some(i + 1);
        }
    }

    for a in ans {
        println!("{}", a.unwrap());
    }
}
