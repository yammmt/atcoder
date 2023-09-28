// https://scrapbox.io/procon-kirokuyou/diverta2019-2_C_-_Successive_Subtraction_(500)
// 34min 教育的
// WA: 教育
// TODO: 処理をまとめて書く

use proconio::input;

fn main() {
    input! {
        n: usize,
        mut an: [i64; n],
    }
    an.sort_unstable();

    let mut ans_row = vec![];
    if an[1] >= 0 {
        // 負の数が一つ以下ならばひたすらマイナスに振り切って最後に y としてプラス側にもっていく
        let mut cur = an[0];
        for i in 1..n - 1 {
            ans_row.push((cur, an[i]));
            cur -= an[i];
        }
        ans_row.push((*an.last().unwrap(), cur));
        let ans_num = *an.last().unwrap() - cur;
        println!("{}", ans_num);
    } else {
        // 負の数が二つ以上ならば二つ目以降の負の数は最も大きい数に足していく (y として選ぶ)
        let mut cur = *an.last().unwrap();
        let mut idx = 1;
        while idx < n - 1 && an[idx] < 0 {
            ans_row.push((cur, an[idx]));
            cur -= an[idx];
            idx += 1;
        }
        let last_x = cur;

        // 正の数は引いていく
        let mut cur = an[0];
        while idx < n - 1 {
            ans_row.push((cur, an[idx]));
            cur -= an[idx];
            idx += 1;
        }
        let last_y = cur;

        ans_row.push((last_x, last_y));
        println!("{}", last_x - last_y);
    }

    for a in &ans_row {
        println!("{} {}", a.0, a.1);
    }
}
