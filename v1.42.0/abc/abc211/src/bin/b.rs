use proconio::input;

fn main() {
    input! {
        s4: [String; 4],
    }

    let mut nums = vec![0; 4];
    for s in &s4 {
        match s.as_str() {
            "H" => nums[0] += 1,
            "2B" => nums[1] += 1,
            "3B" => nums[2] += 1,
            "HR" => nums[3] += 1,
            _ => unreachable!(),
        }
    }

    println!(
        "{}",
        if nums.iter().all(|&a| a == 1) {
            "Yes"
        } else {
            "No"
        }
    );
}
