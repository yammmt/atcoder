use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        n: usize,
        mut lrn: [(usize, usize); n],
    }
    lrn.sort_unstable();

    let mut ans = vec![];
    let mut cur_begin = 0;
    let mut cur_end = 0;
    for (l, r) in lrn {
        if l > cur_end {
            // 新しく
            if cur_begin != 0 {
                ans.push((cur_begin, cur_end));
            }
            cur_begin = l;
            cur_end = r;
        } else {
            // 延長して継続
            cur_end = cur_end.max(r);
        }
    }
    ans.push((cur_begin, cur_end));

    for a in ans {
        println!("{} {}", a.0, a.1);
    }
}
