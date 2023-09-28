use proconio::input;

fn main() {
    input! {
        n: usize,
        abn: [(usize, usize); n],
    }

    let mut events = vec![];
    for ab in abn {
        events.push((ab.0, 1isize));
        events.push((ab.0 + ab.1, -1));
    }
    events.sort_unstable();
    // println!("{:?}", events);

    let mut ans = vec![0; n + 1];
    let mut prev_day = events.first().unwrap().0;
    let mut cur = 0isize;
    for (i, e) in events.iter().enumerate() {
        // println!("{}: {:?}", i, e);
        // println!("  prev_day: {}", prev_day);
        // 日付が変わった場合 (0 人ログイン期間は数えない)
        if i != 0 && e.0 != events[i - 1].0 {
            // println!("  {} += {}", cur, e.0 - prev_day);
            ans[cur as usize] += e.0 - prev_day;
            prev_day = e.0;
        }

        cur += e.1;
    }
    // println!("{:?}", ans);

    for (i, a) in ans.iter().skip(1).enumerate() {
        print!("{}", a);
        if i == ans.len() - 1 {
            println!();
        } else {
            print!(" ");
        }
    }
}
