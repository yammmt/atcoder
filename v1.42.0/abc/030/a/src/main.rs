// -*- coding:utf-8-unix -*-

use proconio::input;
use std::cmp::Ordering;

fn main() {
    input! {
        a: u64,
        b: u64,
        c: u64,
        d: u64,
    }
    println!(
        "{}",
        match (b * c).cmp(&(a * d)) {
            Ordering::Less => "AOKI",
            Ordering::Equal => "DRAW",
            Ordering::Greater => "TAKAHASHI",
        }
    );
}
