use proconio::fastout;
use proconio::input;
use std::collections::BTreeSet;

#[fastout]
fn main() {
    input! {
        n: usize,
        mut tdn: [(usize, usize); n],
    }

    // 区間スケジューリング問題とか座圧に見える
    // ある時刻で処理できるもののうち, 最も早く出ていくものを処理する貪欲でよさそう
    // t, d は巨大だが n はそうでもなく, 時刻更新に O(N) かけても間に合う
    tdn.sort_unstable();
    // 番兵
    tdn.push((2_000_000_000_000_000_000, 2_000_000_000_000_000_000));
    // 突入時刻は無視してよく, 出る時間のみ見る
    let mut available_items: BTreeSet<(usize, usize)> = BTreeSet::new();
    let mut last_time = 0;
    let mut ans = 0;
    for (i, td) in tdn.iter().enumerate() {
        let t = td.0;
        let d = td.1;
        // t=1, d=1 となった場合に一つ印字する？二つ印字する？
        //     => 重複怖いので [), 最後は番兵でさばく
        // 時刻が変化しない場合には取りだし処理をしてはならぬ
        if t != last_time {
            while let Some(cur) = available_items.pop_first() {
                if cur.0 >= last_time {
                    last_time += 1;
                    ans += 1;
                    if last_time >= t {
                        break;
                    }
                }
            }
        }
        // 重複要素を扱うため i を入れてごまかす
        available_items.insert((t + d, i));

        // 前回投入時間を次の処理の起点
        last_time = t;
    }
    // 最後の商品投入後は一つずつ印字判定をする

    println!("{ans}");
}
