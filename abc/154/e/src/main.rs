// -*- coding:utf-8-unix -*-

// 曰く桁 DP
// https://drken1215.hatenablog.com/entry/2020/02/09/225300

use proconio::input;

// 問題文より r は [1, 3]
fn ncr(n: u64, r: u64) -> u64 {
    match r {
        1 => return n,
        2 => return n * (n - 1) / 2,
        3 => return n * (n - 1) * (n - 2) / 6,
        _ => unreachable!()
    }
}

// 上から i 桁目までの値が確定済
// 0 でない数をあと k 個使える
fn solve(vns: &Vec<char>, i: u64, k: u64, is_small: bool) -> u64 {
    if i == vns.len() as u64 {
        // 最終桁まで確定したんだから 1 通り
        if k == 0 {
            return 1;
        } else {
            // ここで k を消費仕切っていない場合は不正
            return 0;
        }
    }

    if k == 0 {
        // 残りの桁をすべて 0 で埋めてやらねばならないため 1 通り
        return 1;
    }

    if is_small {
        // どう組んでも小さくなる数が出来上がる
        // 0 以外の数 9 通りを残りの桁数 vns.len() - 1 桁の中の k 箇所に入れる
        return 9u64.pow(k as u32) * ncr(vns.len() as u64 - i, k);
    } else if vns[i as usize] == '0' {
        // 今の桁を 0 で埋める他ない
        return solve(&vns, i + 1, k, false);
    } else {
        // ここで 0 を使った場合は比較対象より小さい数が出来上がる
        let zero = solve(&vns, i + 1, k, true);
        // 0 を使わずとも今の桁の数より小さい数を使った場合は比較対象より小さい数が出来上がる
        let aida = solve(&vns, i + 1, k - 1, true) * ((vns[i as usize] as u8 - b'1') as u64);
        // vns[i] を使った場合は数の大小は確定しない
        let usei = solve(&vns, i + 1, k - 1, false);

        return zero + aida + usei;
    }
}

fn main() {
    input! {
        n: String,
        k: u64
    }

    let vns = n.chars().collect::<Vec<char>>();
    println!("{}", solve(&vns, 0, k, false));
}
