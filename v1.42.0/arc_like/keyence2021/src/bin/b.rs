use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        mut an: [usize; n],
    }
    an.sort_unstable();
    // println!("{:?}", an);

    let mut ans = 0;
    let mut kleft = k;
    let mut idx = 0;
    for i in 0..an[n - 1] + 1 {
        // println!("i: {}", i);
        let mut streak = 0;
        while idx < n && an[idx] == i {
            streak += 1;
            idx += 1;
        }
        if streak > kleft && idx != n {
            continue;
        }

        if kleft > streak {
            ans += i * (kleft - streak);
            kleft = streak;
        }

        // println!("kleft: {}", kleft);
        if kleft == 0 {
            break;
        }

        if i == an[n - 1] {
            ans += (an[n - 1] + 1) * streak.min(kleft);
        }
    }

    println!("{}", ans);
}
