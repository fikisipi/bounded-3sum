use std::time::Instant;

use three_sum::Solution;

const REPEATS: usize = 5;

fn has_three_sum_quadratic(mut nums: Vec<i32>) -> bool {
    if nums.len() < 3 {
        return false;
    }

    nums.sort_unstable();

    for i in 0..nums.len() - 2 {
        if i > 0 && nums[i] == nums[i - 1] {
            continue;
        }

        if nums[i] > 0 {
            break;
        }

        let mut left = i + 1;
        let mut right = nums.len() - 1;

        while left < right {
            let sum = nums[i] as i64 + nums[left] as i64 + nums[right] as i64;

            if sum == 0 {
                return true;
            }

            if sum < 0 {
                left += 1;
            } else {
                right -= 1;
            }
        }
    }

    false
}

fn make_input(n: usize, span: i32) -> Vec<i32> {
    let mut seed = 0x1234_5678_9abc_def0_u64;
    let mut nums = Vec::with_capacity(n);

    for _ in 0..n {
        seed = seed.wrapping_mul(6_364_136_223_846_793_005).wrapping_add(1);
        let bucket = (seed % (2 * span) as u64) as i32 - span;
        nums.push(4 * bucket + 1);
    }

    nums
}

fn run_case(n: usize, span: i32) {
    let nums = make_input(n, span);
    let min = nums.iter().min().copied().unwrap();
    let max = nums.iter().max().copied().unwrap();
    let universe = max - min + 1;
    let mut convolution_total_ms = 0.0;
    let mut quadratic_total_ms = 0.0;
    let mut convolution = None;
    let mut quadratic = false;

    for _ in 0..REPEATS {
        let started = Instant::now();
        convolution = Solution::has_three_sum(nums.clone());
        convolution_total_ms += started.elapsed().as_secs_f64() * 1_000.0;

        let started = Instant::now();
        quadratic = has_three_sum_quadratic(nums.clone());
        quadratic_total_ms += started.elapsed().as_secs_f64() * 1_000.0;
    }

    let convolution_ms = convolution_total_ms / REPEATS as f64;
    let quadratic_ms = quadratic_total_ms / REPEATS as f64;

    println!(
        "{n},{universe},{convolution_ms:.3},{quadratic_ms:.3},{convolution:?},{quadratic}"
    );
}

fn main() {
    println!("n,universe,convolution_avg_ms,quadratic_avg_ms,convolution_result,quadratic_result");

    for (n, span) in [(2_000, 500), (10_000, 500), (50_000, 500), (50_000, 12_500)] {
        run_case(n, span);
    }
}
