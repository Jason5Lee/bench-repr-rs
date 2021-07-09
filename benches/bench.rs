use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::rc::Rc;
use std::sync::Arc;

#[derive(Debug)]
pub struct Error1;
#[derive(Debug)]
pub struct Error2;
#[derive(Debug)]
pub enum Error3 {
    Error3_1,
    Error3_2,
}
fn create_value<const N: usize>() -> Result<[u8; N], Error1> {
    let mut if_err: Result<(), Error1> = Ok(());
    black_box(&mut if_err);
    if_err?;
    let mut result = [0u8; N];
    black_box(&mut result);
    Ok(result)
}

fn map_value_err_2<const N: usize>() -> Result<[u8; N], Error2> {
    create_value::<N>().map_err(|Error1| Error2)
}

fn map_value_err_3<const N: usize>() -> Result<[u8; N], Error3> {
    create_value::<N>().map_err(|Error1| Error3::Error3_1)
}

fn create_box<const N: usize>() -> Result<Box<[u8; N]>, Error1> {
    let mut if_err: Result<(), Error1> = Ok(());
    black_box(&mut if_err);
    if_err?;
    let mut result = [0u8; N];
    black_box(&mut result);
    Ok(Box::new(result))
}

fn map_box_err_2<const N: usize>() -> Result<Box<[u8; N]>, Error2> {
    create_box::<N>().map_err(|Error1| Error2)
}

fn map_box_err_3<const N: usize>() -> Result<Box<[u8; N]>, Error3> {
    create_box::<N>().map_err(|Error1| Error3::Error3_1)
}

fn create_rc<const N: usize>() -> Result<std::rc::Rc<[u8; N]>, Error1> {
    let mut if_err: Result<(), Error1> = Ok(());
    black_box(&mut if_err);
    if_err?;
    let mut result = [0u8; N];
    black_box(&mut result);
    Ok(std::rc::Rc::new(result))
}

fn map_rc_err_2<const N: usize>() -> Result<std::rc::Rc<[u8; N]>, Error2> {
    create_rc::<N>().map_err(|Error1| Error2)
}

fn map_rc_err_3<const N: usize>() -> Result<std::rc::Rc<[u8; N]>, Error3> {
    create_rc::<N>().map_err(|Error1| Error3::Error3_1)
}

fn create_arc<const N: usize>() -> Result<std::sync::Arc<[u8; N]>, Error1> {
    let mut if_err: Result<(), Error1> = Ok(());
    black_box(&mut if_err);
    if_err?;
    let mut result = [0u8; N];
    black_box(&mut result);
    Ok(std::sync::Arc::new(result))
}

fn map_arc_err_2<const N: usize>() -> Result<std::sync::Arc<[u8; N]>, Error2> {
    create_arc::<N>().map_err(|Error1| Error2)
}

fn map_arc_err_3<const N: usize>() -> Result<std::sync::Arc<[u8; N]>, Error3> {
    create_arc::<N>().map_err(|Error1| Error3::Error3_1)
}

#[allow(clippy::clone_on_copy)]
fn benchmark_struct<const N: usize>(c: &mut Criterion) {
    let mut group = c.benchmark_group(format!("{}: create", N));
    group.bench_function("stack", |b| {
        b.iter(|| {
            let mut a = create_value::<N>();
            black_box(&mut a);
        })
    });
    group.bench_function("box", |b| {
        b.iter(|| {
            let mut a = create_box::<N>();
            black_box(&mut a);
        })
    });
    group.bench_function("rc", |b| {
        b.iter(|| {
            let mut a = create_rc::<N>();
            black_box(&mut a);
        })
    });
    group.bench_function("arc", |b| {
        b.iter(|| {
            let mut a = create_arc::<N>();
            black_box(&mut a);
        })
    });
    drop(group);

    let mut group = c.benchmark_group(format!("{}: map to err2", N));
    group.bench_function("stack", |b| {
        b.iter(|| {
            let mut a = map_value_err_2::<N>();
            black_box(&mut a);
        })
    });
    group.bench_function("box", |b| {
        b.iter(|| {
            let mut a = map_box_err_2::<N>();
            black_box(&mut a);
        })
    });
    group.bench_function("rc", |b| {
        b.iter(|| {
            let mut a = map_rc_err_2::<N>();
            black_box(&mut a);
        })
    });
    group.bench_function("arc", |b| {
        b.iter(|| {
            let mut a = map_arc_err_2::<N>();
            black_box(&mut a);
        })
    });
    drop(group);

    let mut group = c.benchmark_group(format!("{}: map to err3", N));
    group.bench_function("stack", |b| {
        b.iter(|| {
            let mut a = map_value_err_3::<N>();
            black_box(&mut a);
        })
    });
    group.bench_function("box", |b| {
        b.iter(|| {
            let mut a = map_box_err_3::<N>();
            black_box(&mut a);
        })
    });
    group.bench_function("rc", |b| {
        b.iter(|| {
            let mut a = map_rc_err_3::<N>();
            black_box(&mut a);
        })
    });
    group.bench_function("arc", |b| {
        b.iter(|| {
            let mut a = map_arc_err_3::<N>();
            black_box(&mut a);
        })
    });
    drop(group);

    let mut group = c.benchmark_group(format!("{}: struct clone once", N));
    group.bench_function("stack", |b| {
        b.iter(|| {
            let mut a = create_value::<N>().unwrap();
            let mut b = a.clone();
            black_box((&mut a, &mut b));
        })
    });
    group.bench_function("box", |b| {
        b.iter(|| {
            let mut a = create_box::<N>().unwrap();
            let mut b = a.clone();
            black_box((&mut a, &mut b));
        })
    });
    group.bench_function("rc", |b| {
        b.iter(|| {
            let mut a = create_rc::<N>().unwrap();
            let mut b = a.clone();
            black_box((&mut a, &mut b));
        })
    });
    group.bench_function("arc", |b| {
        b.iter(|| {
            let mut a = create_arc::<N>().unwrap();
            let mut b = a.clone();
            black_box((&mut a, &mut b));
        })
    });
    drop(group);

    let mut group = c.benchmark_group(format!("{}: struct clone twice", N));
    group.bench_function("stack", |b| {
        b.iter(|| {
            let mut a = create_value::<N>().unwrap();
            let mut b = a.clone();
            let mut c = a.clone();
            black_box((&mut a, &mut b, &mut c));
        })
    });
    group.bench_function("box", |b| {
        b.iter(|| {
            let mut a = create_box::<N>().unwrap();
            let mut b = a.clone();
            let mut c = a.clone();
            black_box((&mut a, &mut b, &mut c));
        })
    });
    group.bench_function("rc", |b| {
        b.iter(|| {
            let mut a = create_rc::<N>().unwrap();
            let mut b = a.clone();
            let mut c = a.clone();
            black_box((&mut a, &mut b, &mut c));
        })
    });
    group.bench_function("arc", |b| {
        b.iter(|| {
            let mut a = create_arc::<N>().unwrap();
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
    let mut bytes = [b'0'; N];
    black_box(&mut bytes);
    let str_chars = std::str::from_utf8(&bytes[..]).unwrap();

    let mut group = c.benchmark_group(format!("{}: create str", N));

    group.bench_function("box", |b| {
        b.iter(|| {
            let mut v: Box<str> = str_chars.into();
            black_box(&mut v);
        })
    });
    group.bench_function("rc", |b| {
        b.iter(|| {
            let mut v: Rc<str> = str_chars.into();
            black_box(&mut v);
        })
    });
    group.bench_function("arc", |b| {
        b.iter(|| {
            let mut v: Arc<str> = str_chars.into();
            black_box(&mut v);
        })
    });
    group.bench_function("bytes", |b| {
        b.iter(|| {
            let mut v: bytes::Bytes = bytes::Bytes::copy_from_slice(str_chars.as_bytes());
            black_box(&mut v);
        })
    });
    drop(group);

    let mut group = c.benchmark_group(format!("{}: str clone once", N));

    group.bench_function("box", |b| {
        b.iter(|| {
            let mut v: Box<str> = str_chars.into();
            black_box(v.clone());
            black_box(&mut v);
        })
    });
    group.bench_function("rc", |b| {
        b.iter(|| {
            let mut v: Rc<str> = str_chars.into();
            black_box(v.clone());
            black_box(&mut v);
        })
    });
    group.bench_function("arc", |b| {
        b.iter(|| {
            let mut v: Arc<str> = str_chars.into();
            black_box(v.clone());
            black_box(&mut v);
        })
    });
    group.bench_function("bytes", |b| {
        b.iter(|| {
            let mut v: bytes::Bytes = bytes::Bytes::copy_from_slice(str_chars.as_bytes());
            black_box(v.clone());
            black_box(&mut v);
        })
    });
    drop(group);

    let mut group = c.benchmark_group(format!("{}: str clone twice", N));

    group.bench_function("box", |b| {
        b.iter(|| {
            let mut v: Box<str> = str_chars.into();
            black_box(v.clone());
            black_box(v.clone());
            black_box(&mut v);
        })
    });
    group.bench_function("rc", |b| {
        b.iter(|| {
            let mut v: Rc<str> = str_chars.into();
            black_box(v.clone());
            black_box(v.clone());
            black_box(&mut v);
        })
    });
    group.bench_function("arc", |b| {
        b.iter(|| {
            let mut v: Arc<str> = str_chars.into();
            black_box(v.clone());
            black_box(v.clone());
            black_box(&mut v);
        })
    });
    group.bench_function("bytes", |b| {
        b.iter(|| {
            let mut v: bytes::Bytes = bytes::Bytes::copy_from_slice(str_chars.as_bytes());
            black_box(v.clone());
            black_box(v.clone());
            black_box(&mut v);
        })
    });
    drop(group);

    let mut group = c.benchmark_group(format!("{}: str move once", N));

    group.bench_function("box", |b| {
        b.iter(|| {
            let v: Box<str> = str_chars.into();
            let mut v = black_box(v);
            black_box(&mut v);
        })
    });
    group.bench_function("rc", |b| {
        b.iter(|| {
            let v: Rc<str> = str_chars.into();
            let mut v = black_box(v);
            black_box(&mut v);
        })
    });
    group.bench_function("arc", |b| {
        b.iter(|| {
            let v: Arc<str> = str_chars.into();
            let mut v = black_box(v);
            black_box(&mut v);
        })
    });
    group.bench_function("bytes", |b| {
        b.iter(|| {
            let v: bytes::Bytes = bytes::Bytes::copy_from_slice(str_chars.as_bytes());
            let mut v = black_box(v);
            black_box(&mut v);
        })
    });
    drop(group);

    let mut group = c.benchmark_group(format!("{}: str move twice", N));

    group.bench_function("box", |b| {
        b.iter(|| {
            let v: Box<str> = str_chars.into();
            let v = black_box(v);
            let mut v = black_box(v);
            black_box(&mut v);
        })
    });
    group.bench_function("rc", |b| {
        b.iter(|| {
            let v: Rc<str> = str_chars.into();
            let v = black_box(v);
            let mut v = black_box(v);
            black_box(&mut v);
        })
    });
    group.bench_function("arc", |b| {
        b.iter(|| {
            let v: Arc<str> = str_chars.into();
            let v = black_box(v);
            let mut v = black_box(v);
            black_box(&mut v);
        })
    });
    group.bench_function("bytes", |b| {
        b.iter(|| {
            let v: bytes::Bytes = bytes::Bytes::copy_from_slice(str_chars.as_bytes());
            let v = black_box(v);
            let mut v = black_box(v);
            black_box(&mut v);
        })
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
