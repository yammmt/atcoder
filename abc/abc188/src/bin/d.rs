use proconio::input;

fn main() {
    input! {
        n: usize,
        c: i64,
        abcn: [(usize, usize, i64); n],
    }

    let mut events = vec![];
    for abc in &abcn {
        events.push((abc.0, abc.2));
        events.push((abc.1 + 1, -abc.2));
    }
    events.sort_unstable();

    let mut ans = 0;
    let mut cur = 0;
    for i in 0..events.len() - 1 {
        cur += events[i].1;
        ans += cur.min(c) * (events[i + 1].0 as i64 - events[i].0 as i64);
    }
    println!("{}", ans);
}
