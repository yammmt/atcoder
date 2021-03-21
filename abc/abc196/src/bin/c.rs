use proconio::input;

fn digit_len(mut n: u64) -> u32 {
    let mut ret = 0;
    while n > 0 {
        ret += 1;
        n /= 10;
    }
    ret
}

fn main() {
    input! {
        n: u64,
    }

    let mut ans = 0;
    for back in 1..n + 1 {
        let mut front = back;
        for _ in 0..digit_len(front) {
            front *= 10;
        }
        if front + back <= n {
            ans += 1;
        } else {
            break;
        }
    }

    println!("{}", ans);
}
