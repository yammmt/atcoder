use proconio::input;

const A_MAX: usize = 300000;

fn main() {
    input! {
        n: usize,
        an: [usize; n],
    }

    // 左の候補数を覚える
    // [1, 2, 3, 1, 4, 1] として, ans の増え方は
    // [0, 1, 1, 0, 2, 0]
    // すべての数字に対してこれを随時数えると TLE だが, 更新が必要なのは a[i] だけで高速化できそう
    let mut ans = 0;
    let mut left_cnt = vec![0; A_MAX + 1];
    let mut confirmed_num = vec![0; A_MAX + 1];
    let mut last_appeared = vec![-1isize; A_MAX + 1];
    for (i, &a) in an.iter().enumerate() {
        if last_appeared[a] > -1 {
            confirmed_num[a] += left_cnt[a] * (i - last_appeared[a] as usize - 1);
            ans += confirmed_num[a];
        }

        left_cnt[a] += 1;
        last_appeared[a] = i as isize;
    }

    println!("{ans}");
}
