use criterion::{black_box, criterion_group, criterion_main, Criterion};
use deku::prelude::*;
use palantir_types::parts::string::*;
use palantir_types::primitives::character::Character;
use std::convert::TryFrom;

pub fn bench_string_8_deserialize(c: &mut Criterion) {
    let mut data = vec![7u8];
    data.append(&mut vec![0x30u8; 7]);

    c.bench_function("String8 deserialize", |b| {
        b.iter(|| String8::from_bytes((black_box(data.as_ref()), black_box(0))))
    });
}

pub fn bench_string_8_serialize(c: &mut Criterion) {
    let s = String8 {
        count: 7,
        data: vec![Character::try_from(0x30).unwrap(); 7],
    };
    c.bench_function("String8 serialize", |b| b.iter(|| s.to_bytes()));
}

pub fn bench_string_16_deserialize(c: &mut Criterion) {
    let mut data = vec![15u8];
    data.append(&mut vec![0x30u8; 15]);

    c.bench_function("String16 deserialize", |b| {
        b.iter(|| String8::from_bytes((black_box(data.as_ref()), black_box(0))))
    });
}

pub fn bench_string_16_serialize(c: &mut Criterion) {
    let s = String8 {
        count: 15,
        data: vec![Character::try_from(0x30).unwrap(); 15],
    };
    c.bench_function("String16 serialize", |b| b.iter(|| s.to_bytes()));
}

pub fn bench_string_256_deserialize(c: &mut Criterion) {
    let mut data = vec![255u8];
    data.append(&mut vec![0x30u8; 255]);

    c.bench_function("String256 deserialize", |b| {
        b.iter(|| String8::from_bytes((black_box(data.as_ref()), black_box(0))))
    });
}

pub fn bench_string_256_serialize(c: &mut Criterion) {
    let s = String8 {
        count: 255,
        data: vec![Character::try_from(0x30).unwrap(); 255],
    };
    c.bench_function("String256 serialize", |b| b.iter(|| s.to_bytes()));
}

criterion_group!(
    benches,
    bench_string_8_deserialize,
    bench_string_8_serialize,
    bench_string_16_deserialize,
    bench_string_16_serialize,
    bench_string_256_deserialize,
    bench_string_256_serialize,
);
criterion_main!(benches);
