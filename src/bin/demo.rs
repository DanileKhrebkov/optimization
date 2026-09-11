use broken_app::{algo, concurrency, leak_buffer, normalize, sum_even, average_positive};

fn main() {
    println!("=== broken-app Demo (Fixed) ===\n");
    let nums = [1, 2, 3, 4];
    println!("sum_even({:?}): {}", &nums, sum_even(&nums));
    let data = [1_u8, 0, 2, 3];
    println!("leak_buffer: {}", leak_buffer(&data));
    println!("normalize: '{}'", normalize(" Hello World "));
    println!("fast_fib(20): {}", algo::fast_fib(20));
    println!("dedup: {:?}", algo::optimized_dedup(&[1, 2, 2, 3, 1, 4, 4]));
    println!("avg_pos: {:.2}", average_positive(&[-5, 5, 15]));
    println!("race_increment(1000, 10): {}", concurrency::race_increment(1000, 10));
    println!("global: {}", concurrency::read_current_counter());
}