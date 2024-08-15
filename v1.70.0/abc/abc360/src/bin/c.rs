use proconio::fastout;
use proconio::input;
use proconio::marker::Usize1;

#[fastout]
fn main() {
    input! {
        n: usize,
        an: [Usize1; n],
        wn: [usize; n],
    }

    let mut wan = Vec::with_capacity(n);
    for i in 0..n {
        wan.push((wn[i], an[i]));
    }
    wan.sort_unstable();
    // w 降順にする,重いものは動かしたくないので
    wan.reverse();

    let mut ans = 0;
    let mut appeared = vec![false; n];
    for (w, a) in wan {
        if appeared[a] {
            ans += w;
        }

        appeared[a] = true;
    }

    println!("{ans}");
}
