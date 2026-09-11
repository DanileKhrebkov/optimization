use broken_app::{algo, concurrency, leak_buffer, normalize, sum_even, average_positive};
use serial_test::serial;

#[test] fn sums_even_numbers() { assert_eq!(sum_even(&[1,2,3,4]), 6); }
#[test] fn sums_even_with_negative() { assert_eq!(sum_even(&[-2_i64,-1,0,1,2,3]), 0); }
#[test] fn sums_even_all_negative_even() { assert_eq!(sum_even(&[-2_i64,-4,-6]), -12); }
#[test] fn sums_even_empty() { assert_eq!(sum_even(&[]), 0); }

#[test] fn counts_non_zero() { assert_eq!(leak_buffer(&[0,1,0,2,3]), 3); }
#[test] fn counts_all_zero() { assert_eq!(leak_buffer(&[0,0,0]), 0); }

#[test] fn dedup_basic() { assert_eq!(algo::optimized_dedup(&[5,5,1,2,2,3]), vec![1,2,3,5]); }
#[test] fn dedup_empty() { assert_eq!(algo::optimized_dedup(&[]), Vec::<u64>::new()); }
#[test] fn dedup_all_same() { assert_eq!(algo::optimized_dedup(&[5,5,5]), vec![5]); }

#[test] fn fib_small() {
    assert_eq!(algo::fast_fib(0), 0);
    assert_eq!(algo::fast_fib(10), 55);
    assert_eq!(algo::fast_fib(20), 6765);
}

#[test] fn normalize_simple() { assert_eq!(normalize(" Hello World "), "helloworld"); }
#[test] fn normalize_with_tabs() { assert_eq!(normalize("Hello\tWorld\nTest"), "helloworldtest"); }
#[test] fn normalize_multiple_spaces() { assert_eq!(normalize("Hello   World"), "helloworld"); }
#[test] fn normalize_empty() { assert_eq!(normalize(""), ""); }

#[test] fn avg_positive_basic() {
    assert!((average_positive(&[-5_i64,5,15]) - 10.0).abs() < f64::EPSILON);
}
#[test] fn avg_positive_all_negative() { assert_eq!(average_positive(&[-1_i64,-2]), 0.0); }
#[test] fn avg_positive_empty() { assert_eq!(average_positive(&[]), 0.0); }

#[test] fn concurrent_4x1000() { assert_eq!(concurrency::race_increment(1000, 4), 4000); }
#[test] fn concurrent_8x10000() { assert_eq!(concurrency::race_increment(10000, 8), 80000); }

#[test] #[serial]
fn counter_reset() {
    concurrency::reset_counter();
    assert_eq!(concurrency::read_current_counter(), 0);
    concurrency::increment_global_counter(100);
    assert_eq!(concurrency::read_current_counter(), 100);
    concurrency::reset_counter();
    assert_eq!(concurrency::read_current_counter(), 0);
}

#[test] #[serial]
fn global_independent_from_local() {
    concurrency::reset_counter();
    let r = concurrency::race_increment(100, 2);
    assert_eq!(r, 200);
    assert_eq!(concurrency::read_current_counter(), 0);
}