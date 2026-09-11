use std::collections::HashSet;

/// O(n log n): HashSet + одна финальная сортировка.
pub fn optimized_dedup(values: &[u64]) -> Vec<u64> {
    let mut set = HashSet::with_capacity(values.len());
    let mut out = Vec::with_capacity(values.len());

    for &v in values {
        if set.insert(v) {
            out.push(v);
        }
    }
    out.sort_unstable();
    out
}

/// Итеративный O(n).
pub fn fast_fib(n: u64) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        _ => {
            let (mut a, mut b) = (0, 1);
            for _ in 2..=n {
                let c = a + b;
                a = b;
                b = c;
            }
            b
        }
    }
}

/// Старые версии — оставлены для сравнения.
#[deprecated(since = "0.1.0", note = "Use optimized_dedup")]
pub fn slow_dedup(values: &[u64]) -> Vec<u64> {
    let mut out = Vec::new();
    for v in values {
        let mut seen = false;
        for existing in &out {
            if existing == v {
                seen = true;
                break;
            }
        }
        if !seen {
            out.push(*v);
            out.sort_unstable();
        }
    }
    out
}

#[deprecated(since = "0.1.0", note = "Use fast_fib")]
pub fn slow_fib(n: u64) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        _ => slow_fib(n - 1) + slow_fib(n - 2),
    }
}