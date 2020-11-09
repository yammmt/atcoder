use proconio::input;

fn main() {
    input! {
        mut n: u64,
    }

    let mut keta = vec![];
    let mut csum = 0;
    while n > 0 {
        let dv = n % 10;
        csum += dv;
        keta.push(dv);
        n /= 10;
    }
    let csum = csum;
    // println!("{:?}", keta);

    let mut ans = 19;
    for bit_row in 0..2u32.pow(keta.len() as u32) {
        let mut selected = vec![];
        for i in 0..keta.len() {
            if bit_row & (1 << i) > 0 {
                selected.push(i);
            }
        }
        // println!("selected: {:?}", selected);
        if selected.len() >= ans || selected.len() == keta.len() {
            continue;
        }

        let mut csum_cloned = csum;
        for s in &selected {
            csum_cloned -= keta[*s as usize];
        }
        // println!("csc: {}", csum_cloned);
        if csum_cloned % 3 == 0 {
            ans = selected.len();
        }
    }

    if ans != 19 {
        println!("{}", ans);
    } else {
        println!("-1");
    }
}
