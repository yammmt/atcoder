use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars; h],
    }
    let d = 10u64.pow(9) + 7;

    let mut lamph = vec![vec![0; w]; h];
    let mut lampw = vec![vec![0; w]; h];
    let mut lamphw = vec![vec![0; w]; h];
    for i in 0..h {
        let mut cur = 0;
        for j in 0..w {
            if s[i][j] == '.' {
                lampw[i][j] = cur;
                cur += 1;
            } else {
                cur = 0;
            }
        }
    }
    for j in 0..w {
        let mut cur = 0;
        for i in 0..h {
            if s[i][j] == '.' {
                lamph[i][j] = cur;
                cur += 1;
            } else {
                cur = 0;
            }
        }
    }
    for i in 0..h {
        if i == 0 {
            for j in 0..w {
                // println!("start from ({}, {})", i, j);
                let mut added = 0;
                let mut cur = 0;
                while i + added < h && j + added < w {
                    // println!("({}, {})", i + added, j + added);
                    if s[i + added][j + added] == '.' {
                        lamphw[i + added][j + added] = cur;
                        cur += 1;
                    } else {
                        cur = 0;
                    }
                    added += 1;
                }
            }
        } else {
            // w == 1 のみ見れば良い
            let j = 0;
            let mut added = 0;
            let mut cur = 0;
            while i + added < h && j + added < w {
                if s[i + added][j + added] == '.' {
                    lamphw[i + added][j + added] = cur;
                    cur += 1;
                } else {
                    cur = 0;
                }
                added += 1;
            }
        }
    }
    // println!("h: {:?}", lamph);
    // println!("w: {:?}", lampw);
    // println!("hw: {:?}", lamphw);
    let mut lampall = vec![vec![0; w]; h];
    for i in 0..h {
        for j in 0..w {
            lampall[i][j] = lamph[i][j] + lampw[i][j] + lamphw[i][j];
        }
    }
    lampall[0][0] = 1;
    println!("{:?}", lampall);

    // 累積和もっておかないと計算バグるのでは
    let mut dp = vec![vec![0; w]; h];
    dp[0][0] = 1;
    for i in 0..h {
        for j in 0..w {
            println!("{}, {}", i, j);
            if i + 1 < h && s[i + 1][j] == '.' {
                println!("i + 1");
                dp[i + 1][j] = (dp[i + 1][j] + lampall[i][j]) % d;
            }
            if j + 1 < w && s[i][j + 1] == '.' {
                dp[i][j + 1] = (dp[i][j + 1] + lampall[i][j]) % d;
                println!("j + 1");
            }
            if i + 1 < h && j + 1 < w && s[i + 1][j + 1] == '.' {
                dp[i + 1][j + 1] = (dp[i + 1][j + 1] + lampall[i][j]) % d;
                println!("ij + 1");
            }
            println!("{:?}", dp);
        }
    }
    println!("{:?}", dp);

    println!("{}", dp[h - 1][w - 1]);
}
