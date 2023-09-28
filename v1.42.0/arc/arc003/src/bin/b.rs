use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        sn: [Chars; n],
    }

    let mut ans = vec![];
    for s in &sn {
        let mut ss = s.clone();
        ss.reverse();
        ans.push(ss);
    }
    ans.sort_unstable();

    for a in &ans {
        let mut aa = a.clone();
        aa.reverse();
        println!("{}", aa.iter().collect::<String>());
    }
}
