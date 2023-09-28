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
        n: usize,
        m: usize,
        q: usize,
        mut wvn: [(i64, i64); n],
        xm: [i64; m],
        qq: [(usize, usize); q],
    }

    wvn.sort_by(|a, b| {
        a.1.cmp(&b.1)
    });
    wvn.reverse();
    // println!("{:?}", wvn);

    // 50個の箱に50商品を入れるとして50!通り
    // 価値が大きいものから入れられる箱に入れていく
    for query in &qq {
        let mut boxes = vec![];
        for i in 0..m {
            if i + 1 >= query.0 && i < query.1 {
                continue;
            }

            boxes.push(xm[i]);
        }
        // println!("{:?}", boxes);
        if boxes.is_empty() {
            println!("0");
            continue;
        }

        boxes.sort();

        let mut boxes_val = vec![0; boxes.len()];
        for nimotsu_i in 0..wvn.len() {
            let mut min_i = lower_bound(&boxes, wvn[nimotsu_i].0);
            while min_i < boxes_val.len() && boxes_val[min_i] != 0  {
                min_i += 1;
            }
            if min_i < boxes_val.len() {
                boxes_val[min_i] = wvn[nimotsu_i].1;
            }
        }
        println!("{}", boxes_val.iter().sum::<i64>());
    }
}
