// 4min

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        abn: [(isize, isize); n],
        cdm: [(isize, isize); m],
    }

    for ab in &abn {
        let mut ansd = std::isize::MAX;
        let mut ansp = 0;
        #[allow(clippy::needless_range_loop)]
        for j in 0..m {
            let curd = (cdm[j].0 - ab.0).abs() + (cdm[j].1 - ab.1).abs();
            if curd < ansd {
                ansd = curd;
                ansp = j + 1;
            }
        }
        println!("{}", ansp);
    }
}
