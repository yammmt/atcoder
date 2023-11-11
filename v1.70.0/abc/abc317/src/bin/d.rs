use proconio::input;

static DUMMY: usize = std::usize::MAX / 4;

fn main() {
    input! {
        n: usize,
        // T, A, 議席数
        xyzn: [(usize, usize, usize); n],
    }
    let mut zsum = 0;
    for xyz in &xyzn {
        zsum += xyz.2;
    }

    // dp[i]: 追加で i 議席獲得するために必要な最小寝返り人数
    let mut dp = vec![DUMMY; zsum + 1];
    dp[0] = 0;
    for xyz in &xyzn {
        let x = xyz.0;
        let y = xyz.1;
        let z = xyz.2;
        if x > y {
            // 必要なし
            continue;
        }

        let mut cur = dp.clone();
        for j in 0..zsum {
            if dp[j] == DUMMY {
                continue;
            }

            cur[j + z] = cur[j + z].min(dp[j] + (x + y + 1) / 2 - x);
        }
        dp = cur;
    }
    // println!("{:?}", dp);

    let mut z_cur = 0;
    for xyz in &xyzn {
        if xyz.0 > xyz.1 {
            z_cur += xyz.2;
        }
    }

    let betray_min = if z_cur > zsum / 2 {
        0
    } else {
        zsum / 2 + 1 - z_cur
    };
    let mut ans = DUMMY;
    for d in dp.iter().take(zsum + 1).skip(betray_min) {
        ans = ans.min(*d);
    }

    println!("{ans}");
}
