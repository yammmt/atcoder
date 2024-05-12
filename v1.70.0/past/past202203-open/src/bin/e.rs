use proconio::input;

fn is_leap_year(y: usize) -> bool {
    if y % 400 == 0 {
        true
    } else {
        y % 4 == 0 && y % 100 != 0
    }
}

fn main() {
    input! {
        s: String,
    }
    let d_limits = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

    let vs = s.split('/').collect::<Vec<&str>>();
    let mut y = vs[0].parse::<usize>().unwrap();
    let mut m = vs[1].parse::<usize>().unwrap();
    let mut d = vs[2].parse::<usize>().unwrap();

    // 365 日を 10,000 年試しても大した数にならないので
    loop {
        let mut cnt = vec![0; 10];
        let mut yy = y;
        for _ in 0..4 {
            cnt[yy % 10] += 1;
            yy /= 10;
        }
        let mut mm = m;
        for _ in 0..2 {
            cnt[mm % 10] += 1;
            mm /= 10;
        }
        let mut dd = d;
        for _ in 0..2 {
            cnt[dd % 10] += 1;
            dd /= 10;
        }

        if cnt.iter().filter(|&&d| d > 0).count() <= 2 {
            println!("{y}/{:02}/{:02}", m, d);
            return;
        }

        d += 1;
        if d > d_limits[m - 1] && !(m == 2 && is_leap_year(y) && d == 28) {
            d = 1;
            m += 1;
        }
        if m > 12 {
            m = 1;
            y += 1;
        }
    }
}
