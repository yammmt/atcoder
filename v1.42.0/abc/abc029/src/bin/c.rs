use proconio::input;

fn main() {
    input! {
        n: u32,
    }

    let mut ans = vec![];
    for i in 0..3u64.pow(n) {
        let mut cur = vec![];
        let mut ii = i;
        for _ in 0..n {
            cur.push(match ii % 3 {
                0 => 'a',
                1 => 'b',
                2 => 'c',
                _ => unreachable!(),
            });
            ii /= 3;
        }
        ans.push(cur.iter().collect::<String>());
    }
    ans.sort_unstable();

    ans.iter().for_each(|a| println!("{}", a));
}
