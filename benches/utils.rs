#[macro_use]
extern crate bencher;

use bencher::Bencher;

fn get_n() -> u64 {
    285889432707005401
}

fn alternating_digit_sum_safe(bench: &mut Bencher) {
    let n = get_n();
    bench.iter(|| {
        n.to_string()
            .chars()
            .map(|char| i64::from(char.to_digit(10).unwrap()))
            .enumerate()
            .fold(0, |acc, (i, digit)| {
                acc + ((if i % 2 == 0 { 1 } else { -1 }) * digit)
            })
    })
}

fn alternating_digit_sum_unsafe(bench: &mut Bencher) {
    let n = get_n();
    bench.iter(|| {
        n.to_string()
            .chars()
            .map(|char| unsafe { i64::from(char.to_digit(10).unwrap_unchecked()) })
            .enumerate()
            .fold(0, |acc, (i, digit)| {
                acc + ((if i % 2 == 0 { 1 } else { -1 }) * digit)
            })
    })
}

fn digit_sum_safe(bench: &mut Bencher) {
    let n = get_n();
    bench.iter(|| {
        n.to_string()
            .chars()
            .map(|char| u64::from(char.to_digit(10).unwrap()))
            .sum::<u64>()
    })
}

fn digit_sum_unsafe(bench: &mut Bencher) {
    let n = get_n();
    bench.iter(|| {
        n.to_string()
            .chars()
            .map(|char| unsafe { u64::from(char.to_digit(10).unwrap_unchecked()) })
            .sum::<u64>()
    })
}

fn last_digit_safe(bench: &mut Bencher) {
    let n = get_n();
    bench.iter(|| u64::from(n.to_string().chars().last().unwrap().to_digit(10).unwrap()))
}

fn last_digit_unsafe(bench: &mut Bencher) {
    let n = get_n();
    bench.iter(|| {
        u64::from(unsafe {
            n.to_string()
                .chars()
                .last()
                .unwrap_unchecked()
                .to_digit(10)
                .unwrap_unchecked()
        })
    })
}

benchmark_group!(
    benches,
    alternating_digit_sum_safe,
    alternating_digit_sum_unsafe,
    digit_sum_safe,
    digit_sum_unsafe,
    last_digit_safe,
    last_digit_unsafe,
);
benchmark_main!(benches);
