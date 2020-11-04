use proconio::input;

// 8min

fn main() {
    input! {
        _n: usize,
        _m: usize,
        storename: String,
        kitname: String,
    }

    let vs = storename.chars().collect::<Vec<char>>();
    let vk = kitname.chars().collect::<Vec<char>>();

    let mut vcnts = vec![0; 26];
    let mut vcntk = vec![0; 26];
    for c in &vs {
        vcnts[(*c as u8 - b'A') as usize] += 1;
    }
    for c in &vk {
        vcntk[(*c as u8 - b'A') as usize] += 1;
    }
    let mut ans = 0;
    for i in 0..26 {
        if vcnts[i] == 0 {
            continue;
        }

        if vcnts[i] > 0 && vcntk[i] == 0 {
            println!("-1");
            return;
        }

        let mut required = vcnts[i] / vcntk[i];
        if vcnts[i] % vcntk[i] != 0 {
            required += 1;
        }
        ans = ans.max(required);
    }
    println!("{}", ans);
}
