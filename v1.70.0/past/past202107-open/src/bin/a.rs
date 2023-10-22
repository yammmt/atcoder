use proconio::input;

fn main() {
    input! {
        mut s: u64,
    }

    let chk = s % 10;
    s /= 10;

    let mut vo = vec![];
    let mut ve = vec![];
    let mut now_even = true;
    while s > 0 {
        let r = s % 10;
        s /= 10;
        if now_even {
            ve.push(r);
        } else {
            vo.push(r);
        }
        now_even = !now_even;
    }

    let mut cur = vo.iter().sum::<u64>();
    cur *= 3;
    cur += ve.iter().sum::<u64>();

    println!("{}", if cur % 10 == chk { "Yes" } else { "No" });
}
