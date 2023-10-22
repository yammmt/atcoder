use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        n: i128,
    }

    // わからん
    // 3^38 > 10^18
    // 使える数字の組は 37 個を使う/使わないと符号の組, M は 100 もいかない
    // 2^37 > 10^9 であり TLE, せいぜい 2^27 が限界

    // sample2 は 3 の冪数を順序よく並べているが偶然？
    // 値をつなげて符号を +/-/0 の三択でどうにかなる？
    // 9193
    // 1
    // 3
    // 9
    // 27
    // -81
    // -243
    // 729
    // 2187
    // 6561
    // しても 3^32 でようやく 10^15 を超えるので全探索では無理
    // 超えるまで足す => 超えない最大の数を引く or 消すで作れる？
    // 手元で 20 とか 25 を試すとできそう なんでや

    let mut cur = 1;
    let mut three = 3;
    let mut threes = HashSet::new();
    threes.insert(1);
    while cur < n {
        cur += three;
        threes.insert(three);
        three *= 3;
    }
    let mut decreasable_nums = vec![];
    for t in &threes {
        // 消滅
        decreasable_nums.push(-t);
        // 符号反転
        decreasable_nums.push(-t * 2);
    }
    decreasable_nums.sort_unstable();

    // vec 削除は遅い処理だが桁数少ないので妥協
    // println!("cur: {cur}");
    // println!("dn: {:?}", decreasable_nums);

    while cur != n {
        // cur + decreasable_nums[pass] < n
        // cur + decreasable_nums[fails] >= n
        // n = 2 で死ぬ
        let mut pass = decreasable_nums.len() - 1;
        let mut fail = 0;
        while pass - fail > 1 {
            let mid = (pass + fail) / 2;
            if cur + decreasable_nums[mid] >= n {
                pass = mid;
            } else {
                fail = mid;
            }
        }
        // println!("f: {fail}, p: {pass}");
        // println!("{} {}", decreasable_nums[fail], decreasable_nums[pass]);
        cur += decreasable_nums[pass];
        // println!("new cur: {cur}");
        if threes.contains(&-decreasable_nums[pass]) {
            threes.remove(&-decreasable_nums[pass]);
        } else {
            threes.remove(&(-decreasable_nums[pass] / 2));
            threes.insert(decreasable_nums[pass] / 2);
        }

        let mut dn_new = vec![];
        for i in 0..decreasable_nums.len() {
            if decreasable_nums[i].abs() != decreasable_nums[pass].abs() {
                dn_new.push(decreasable_nums[i]);
            }
        }
        decreasable_nums = dn_new;
    }

    let mut ans = vec![];
    for t in &threes {
        ans.push(*t);
    }
    ans.sort_unstable();
    println!("{}", ans.len());
    for i in 0..ans.len() {
        print!("{}", ans[i]);
        if i == ans.len() - 1 {
            println!();
        } else {
            print!(" ");
        }
    }
}
