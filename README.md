# broken-app: Исправление багов и оптимизация

## Обзор

Проект `broken-app` содержал 8 дефектов: UB через `unsafe`, утечки памяти,
гонки данных, логические ошибки и экспоненциальные/квадратичные алгоритмы.
Все дефекты исправлены, весь `unsafe` удалён.

## Найденные баги и исправления

| # | Файл | Функция | Категория | Исправление |
|---|------|---------|-----------|-------------|
| 1 | src/lib.rs | sum_even | UB (off-by-one) | values.iter().filter().sum() |
| 2 | src/lib.rs | leak_buffer | Утечка | input.iter().filter().count() |
| 3 | src/lib.rs | use_after_free | UB | Удалена |
| 4 | src/lib.rs | average_positive | Логика | Деление на count положительных |
| 5 | src/lib.rs | normalize | Логика | split_whitespace().concat() |
| 6 | src/concurrency.rs | race_increment | Гонка | Arc<AtomicU64> |
| 7 | src/algo.rs | slow_fib | Перф | fast_fib O(n) |
| 8 | src/algo.rs | slow_dedup | Перф | optimized_dedup O(n log n) |

## Верификация

| Инструмент | Лог | Вердикт |
|-----------|-----|---------|
| Miri (broken) | artifacts/01_miri_broken.txt | 🔴 UB найден |
| Miri (fixed) | artifacts/05_miri_after.txt | ✅ OK |
| ASan | artifacts/06_asan_after.txt | ✅ OK |
| TSan (after) | artifacts/07_tsan_after.txt | ✅ OK |
| TSan (before) | artifacts/08_tsan_before.txt | 🔴 Гонка найдена |
| Valgrind | artifacts/09_valgrind.txt | ✅ OK |

## Производительность: до/после

Числа из `artifacts/10_before_after_FINAL.txt`:

### Fibonacci

| n | Before | After | Ускорение |
|---|--------|-------|-----------|
| 20 | ~20 µs | ~12 ns | ~1700x |
| 25 | ~215 µs | ~15 ns | ~14000x |
| 32 | ~секунды | ~19 ns | огромное |

### Dedup

| Размер | Before | After | Ускорение |
|--------|--------|-------|-----------|
| 100 | ~7 µs | ~2 µs | ~3x |
| 500 | ~123 µs | ~11 µs | ~11x |
| 1000 | ~479 µs | ~23 µs | ~21x |
| 2000 | ~1882 µs | ~46 µs | ~41x |

## Как воспроизвести

```bash
cargo test
cargo bench --bench before_after
cargo +nightly miri test
RUSTFLAGS="-Zsanitizer=address" cargo +nightly test -Zbuild-std --target x86_64-unknown-linux-gnu
RUSTFLAGS="-Zsanitizer=thread" cargo +nightly test -Zbuild-std --target x86_64-unknown-linux-gnu concurrent_8x10000
RUSTFLAGS="-Zsanitizer=thread" cargo +nightly test -Zbuild-std --target x86_64-unknown-linux-gnu broken_race_increment -- --ignored --nocapture
valgrind --leak-check=full ./target/release/demo