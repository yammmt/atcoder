// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        d: usize,
    }
    println!(
        "{}",
        match d {
            25 => "Christmas",
            24 => "Christmas Eve",
            23 => "Christmas Eve Eve",
            _ => "Christmas Eve Eve Eve",
        }
    );
}
