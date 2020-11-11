use proconio::input;

fn main() {
    input! {
        n: usize,
        k: isize,
        mut an: [isize; n],
    }

    for _ in 0..k {
        let mut imos = vec![0; n];
        for i in 0..n {
            // println!("i: {}", i);
            imos[((i as isize - an[i]).max(0)) as usize] += 1;
            let subp = (i + an[i] as usize + 1) as usize;
            if subp < n {
                imos[subp] -= 1;
            }
        }
        // println!("{:?}", imos);
        let mut cur = 0;
        for i in 0..n {
            cur += imos[i];
            an[i] = cur;
        }

        if an.iter().all(|a| *a == n as isize) {
            break;
        }
    }

    for i in 0..n {
        print!("{}", an[i]);
        if i < n - 1 {
            print!(" ");
        } else {
            println!();
        }
    }
}
