pub mod algo;
pub mod algo_broken;
pub mod concurrency;

/// Сумма чётных. Без unsafe.
pub fn sum_even(values: &[i64]) -> i64 {
    values.iter().filter(|&&v| v % 2 == 0).sum()
}

/// Счёт ненулевых байтов. Без утечки.
pub fn leak_buffer(input: &[u8]) -> usize {
    input.iter().filter(|&&b| b != 0).count()
}

/// Нормализация: убирает все пробельные (включая \t, \n), lowercase.
pub fn normalize(input: &str) -> String {
    input.split_whitespace().collect::<Vec<_>>().concat().to_lowercase()
}

/// Среднее только положительных. 0.0 если нет положительных.
pub fn average_positive(values: &[i64]) -> f64 {
    let positives: Vec<&i64> = values.iter().filter(|&&v| v > 0).collect();
    if positives.is_empty() {
        return 0.0;
    }
    positives.iter().map(|&&v| v as f64).sum::<f64>() / positives.len() as f64
}