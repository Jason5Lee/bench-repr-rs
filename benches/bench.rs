use criterion::{black_box, criterion_group, criterion_main, Criterion, BatchSize};
use std::rc::Rc;
use std::sync::Arc;
use bytes::Bytes;
use mimalloc::MiMalloc;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

#[allow(clippy::clone_on_copy)]
fn benchmark_struct<const N: usize>(c: &mut Criterion) {
    let mut group = c.benchmark_group(format!("{}: create", N));
    group.bench_function("stack", |b| {
        b.iter(|| {
            let mut a = [0u8; N];
            black_box(&mut a);
        })
    });
    group.bench_function("box", |b| {
        b.iter(|| {
            let mut a = Box::new([0u8; N]);
            black_box(&mut a);
        })
    });
    group.bench_function("rc", |b| {
        b.iter(|| {
            let mut a = Rc::new([0u8; N]);
            black_box(&mut a);
        })
    });
    group.bench_function("arc", |b| {
        b.iter(|| {
            let mut a = Arc::new([0u8; N]);
            black_box(&mut a);
        })
    });
    drop(group);

    let mut group = c.benchmark_group(format!("{}: struct clone once", N));
    group.bench_function("stack", |b| {
        b.iter(|| {
            let mut a = [0u8; N];
            let mut b = a.clone();
            black_box((&mut a, &mut b));
        })
    });
    group.bench_function("box", |b| {
        b.iter(|| {
            let mut a = Box::new([0u8; N]);
            let mut b = a.clone();
            black_box((&mut a, &mut b));
        })
    });
    group.bench_function("rc", |b| {
        b.iter(|| {
            let mut a = Rc::new([0u8; N]);
            let mut b = a.clone();
            black_box((&mut a, &mut b));
        })
    });
    group.bench_function("arc", |b| {
        b.iter(|| {
            let mut a = Arc::new([0u8; N]);
            let mut b = a.clone();
            black_box((&mut a, &mut b));
        })
    });
    drop(group);

    let mut group = c.benchmark_group(format!("{}: struct clone twice", N));
    group.bench_function("stack", |b| {
        b.iter(|| {
            let mut a = [0u8; N];
            let mut b = a.clone();
            let mut c = a.clone();
            black_box((&mut a, &mut b, &mut c));
        })
    });
    group.bench_function("box", |b| {
        b.iter(|| {
            let mut a = Box::new([0u8; N]);
            let mut b = a.clone();
            let mut c = a.clone();
            black_box((&mut a, &mut b, &mut c));
        })
    });
    group.bench_function("rc", |b| {
        b.iter(|| {
            let mut a = Rc::new([0u8; N]);
            let mut b = a.clone();
            let mut c = a.clone();
            black_box((&mut a, &mut b, &mut c));
        })
    });
    group.bench_function("arc", |b| {
        b.iter(|| {
            let mut a = Arc::new([0u8; N]);
            let mut b = a.clone();
            let mut c = a.clone();
            black_box((&mut a, &mut b, &mut c));
        })
    });
    drop(group);
}

#[allow(clippy::drop_copy)]
#[allow(clippy::clone_on_copy)]
fn benchmark_str<const N: usize>(c: &mut Criterion) {
    fn make_str<const N: usize>() -> String {
        let mut s = String::new();
        for _ in 0..N {
            let mut ch = '0';
            black_box(&mut ch);
            s.push(ch);
        }
        s
    }

    let mut group = c.benchmark_group(format!("{}: create str", N));

    group.bench_function("string", |b| {
        b.iter_batched(make_str::<N>, |s| {
            let mut v: String = s;
            black_box(&mut v);
        }, BatchSize::SmallInput);
    });
    group.bench_function("rc", |b| {
        b.iter_batched(make_str::<N>, |s| {
            let mut v: Rc<String> = s.into();
            black_box(&mut v);
        }, BatchSize::SmallInput);
    });
    group.bench_function("arc", |b| {
        b.iter_batched(make_str::<N>, |s| {
            let mut v: Arc<String> = s.into();
            black_box(&mut v);
        }, BatchSize::SmallInput);
    });
    group.bench_function("bytes", |b| {
        b.iter_batched(make_str::<N>, |s| {
            let mut v: Bytes = s.into();
            black_box(&mut v);
        }, BatchSize::SmallInput);
    });
    drop(group);

    let mut group = c.benchmark_group(format!("{}: str clone once", N));

    group.bench_function("string", |b| {
        b.iter_batched(make_str::<N>, |s| {
            let mut v: String = s;
            black_box(v.clone());
            black_box(&mut v);
        }, BatchSize::SmallInput);
    });
    group.bench_function("rc", |b| {
        b.iter_batched(make_str::<N>, |s| {
            let mut v: Rc<String> = s.into();
            black_box(v.clone());
            black_box(&mut v);
        }, BatchSize::SmallInput);
    });
    group.bench_function("arc", |b| {
        b.iter_batched(make_str::<N>, |s| {
            let mut v: Arc<String> = s.into();
            black_box(v.clone());
            black_box(&mut v);
        }, BatchSize::SmallInput);
    });
    group.bench_function("bytes", |b| {
        b.iter_batched(make_str::<N>, |s| {
            let mut v: Bytes = s.into();
            black_box(v.clone());
            black_box(&mut v);
        }, BatchSize::SmallInput);
    });
    drop(group);

    let mut group = c.benchmark_group(format!("{}: str clone twice", N));

    group.bench_function("string", |b| {
        b.iter_batched(make_str::<N>, |s| {
            let mut v: String = s;
            black_box(v.clone());
            black_box(v.clone());
            black_box(&mut v);
        }, BatchSize::SmallInput);
    });
    group.bench_function("rc", |b| {
        b.iter_batched(make_str::<N>, |s| {
            let mut v: Rc<String> = s.into();
            black_box(v.clone());
            black_box(v.clone());
            black_box(&mut v);
        }, BatchSize::SmallInput);
    });
    group.bench_function("arc", |b| {
        b.iter_batched(make_str::<N>, |s| {
            let mut v: Arc<String> = s.into();
            black_box(v.clone());
            black_box(v.clone());
            black_box(&mut v);
        }, BatchSize::SmallInput);
    });
    group.bench_function("bytes", |b| {
        b.iter_batched(make_str::<N>, |s| {
            let mut v: Bytes = s.into();
            black_box(v.clone());
            black_box(v.clone());
            black_box(&mut v);
        }, BatchSize::SmallInput);
    });
    drop(group);
}

fn criterion_benchmark(c: &mut Criterion) {
    benchmark_struct::<8>(c);
    benchmark_str::<8>(c);
    benchmark_struct::<16>(c);
    benchmark_str::<16>(c);
    benchmark_struct::<32>(c);
    benchmark_str::<32>(c);
    benchmark_struct::<40>(c);
    benchmark_str::<40>(c);
    benchmark_struct::<64>(c);
    benchmark_str::<64>(c);
    benchmark_struct::<96>(c);
    benchmark_str::<96>(c);
    benchmark_struct::<128>(c);
    benchmark_str::<128>(c);
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
