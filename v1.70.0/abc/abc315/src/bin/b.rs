use proconio::input;

fn main() {
    input! {
        m: usize,
        dm: [usize; m],
    }

    let date_num: usize = dm.iter().sum();
    let med_date = date_num / 2;
    let mut cur_date_num = 0;
    for (i, d) in dm.iter().enumerate() {
        if cur_date_num + *d > med_date {
            println!("{} {}", i + 1, med_date - cur_date_num + 1);
            return;
        }

        cur_date_num += *d;
    }
}
