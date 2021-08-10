use proconio::input;

fn dfs(cur: u64, n: u64, uses: (bool, bool, bool)) -> usize {
    if cur > n {
        return 0;
    }

    let mut ret = if uses == (true, true, true) { 1 } else { 0 };
    ret += dfs(cur * 10 + 3, n, (true, uses.1, uses.2));
    ret += dfs(cur * 10 + 5, n, (uses.0, true, uses.2));
    ret += dfs(cur * 10 + 7, n, (uses.0, uses.1, true));
    ret
}

fn main() {
    input! {
        n: u64,
    }

    let ans = dfs(0, n, (false, false, false));
    println!("{}", ans);
}
