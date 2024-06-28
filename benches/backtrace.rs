use async_backtrace_caller::Frame;
use criterion::{criterion_group, criterion_main, Criterion};

fn bench_capture(c: &mut Criterion) {
    c.bench_function("capture", |b| {
        b.iter(|| {
            let _ = Frame::capture();
        })
    });
}

fn bench_noop(c: &mut Criterion) {
    c.bench_function("noop", |b| {
        b.iter(|| {
            criterion::black_box({
                let _ = Frame::capture();
                let _ = (|| {
                    let mut sum = 0;
                    for i in 0..200 {
                        sum += i * 2;
                    }
                    let _ = sum;
                })();
            });
        })
    });
}

criterion_group!(benches, bench_capture, bench_noop);
criterion_main!(benches);
