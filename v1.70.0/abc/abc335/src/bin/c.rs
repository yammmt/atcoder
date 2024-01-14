use proconio::fastout;
use proconio::input;
use proconio::marker::Usize1;

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
    }

    let mut ans = vec![];
    for i in (1..=n).rev() {
        ans.push((i as isize, 0isize));
    }

    for _ in 0..q {
        input! {
            qq: usize,
        }
        if qq == 1 {
            input! {
                c: char,
            }
            let mut cur = *ans.last().unwrap();
            match c {
                'R' => cur.0 += 1,
                'L' => cur.0 -= 1,
                'U' => cur.1 += 1,
                'D' => cur.1 -= 1,
                _ => unreachable!(),
            }
            ans.push(cur);
        } else {
            input! {
                p: Usize1,
            }
            let i = ans.len() - p - 1;
            println!("{} {}", ans[i].0, ans[i].1);
        }
    }
}
