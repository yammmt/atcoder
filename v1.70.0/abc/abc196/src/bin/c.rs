use proconio::input;

fn n_digit(n: i64) -> i64 {
    let mut nn = n;
    let mut ret = 0;
    while nn > 0 {
        nn /= 10;
        ret += 1;
    }
    ret
}

fn main() {
    input! {
        n: i64,
    }

    let mut ans = 0;
    for i in 1..1_000_000 {
        let mut cur = i as i64;
        let cur_digit = n_digit(cur);
        for _ in 0..cur_digit {
            cur *= 10;
        }
        cur += i;

        if cur <= n {
            ans += 1;
        } else {
            break;
        }
    }

    println!("{ans}");
}
