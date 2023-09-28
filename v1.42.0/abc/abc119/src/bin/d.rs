// (Editinal) ダミー地点を置くと簡略化できる

use proconio::input;

fn lower_bound(v: &[i64], k: i64) -> usize {
    let mut low: isize = -1;
    let mut high = v.len() as isize;

    while high - low > 1 {
        let mid = (low + high) / 2;
        if v[mid as usize] >= k {
            high = mid;
        } else {
            low = mid;
        }
    }
    high as usize
}

fn main() {
    input! {
        a: usize,
        b: usize,
        q: usize,
        sa: [i64; a], // sorted
        tb: [i64; b], // sorted
        xq: [i64; q],
    }
    // 神社/寺共に東側最短と西側最短を覚えておこうにも x <= 10^10 で TLE

    for x in &xq {
        let mut shrine_e_valid = true;
        let mut shrine_w_valid = true;
        let shrine_e;
        let shrine_w;
        let shrine_e_idx = lower_bound(&sa, *x);
        if shrine_e_idx == sa.len() {
            shrine_e = sa[a - 1];
            shrine_w = sa[a - 1];
            shrine_e_valid = false;
        } else if shrine_e_idx == 0 {
            shrine_e = sa[0];
            shrine_w = sa[0];
            shrine_w_valid = false;
        } else {
            shrine_e = sa[shrine_e_idx];
            shrine_w = sa[shrine_e_idx - 1];
        }

        let mut temple_e_valid = true;
        let mut temple_w_valid = true;
        let temple_e;
        let temple_w;
        let temple_e_idx = lower_bound(&tb, *x);
        if temple_e_idx == tb.len() {
            temple_e = tb[b - 1];
            temple_w = tb[b - 1];
            temple_e_valid = false;
        } else if temple_e_idx == 0 {
            temple_e = tb[0];
            temple_w = tb[0];
            temple_w_valid = false;
        } else {
            temple_e = tb[temple_e_idx];
            temple_w = tb[temple_e_idx - 1];
        }

        // println!("se: {}", shrine_e);
        // println!("sw: {}", shrine_w);
        // println!("te: {}", temple_e);
        // println!("tw: {}", temple_w);
        let mut ans = vec![];
        if shrine_e_valid && temple_e_valid {
            // println!("e e");
            ans.push(shrine_e.max(temple_e) - *x);
        }
        if shrine_w_valid && temple_w_valid {
            // println!("w w");
            ans.push(*x - shrine_w.min(temple_w));
        }
        if shrine_e_valid && temple_w_valid {
            // println!("e w");
            ans.push(shrine_e - temple_w + (shrine_e - *x).min(*x - temple_w));
        }
        if shrine_w_valid && temple_e_valid {
            // println!("w e");
            ans.push(temple_e - shrine_w + (temple_e - *x).min(*x - shrine_w));
        }
        // println!("{:?}", ans);

        println!("{}", ans.iter().min().unwrap());
    }
}
