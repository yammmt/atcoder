use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
    }

    let mut ans = n;
    for _ in 1..k + 1 {
        let mut vi = vec![];
        let mut lower = 0;
        let mut upper = 0;

        let mut cur_n = ans;
        while cur_n > 0 {
            vi.push(cur_n % 10);
            cur_n /= 10;
        }

        vi.sort_unstable();
        for v in &vi {
            lower *= 10;
            lower += *v;
        }

        vi.reverse();
        for v in &vi {
            upper *= 10;
            upper += *v;
        }

        ans = upper - lower;
    }

    println!("{}", ans);
}
