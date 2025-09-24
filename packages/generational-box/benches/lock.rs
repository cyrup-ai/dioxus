#![allow(unused)]
use std::hint::black_box;

use criterion::{Criterion, criterion_group, criterion_main};
use generational_box::*;

fn create<S: Storage<u32>>(owner: &Owner<S>) -> GenerationalBox<u32, S> {
    owner.insert(0)
}

fn set_read<S: Storage<u32>>(signal: GenerationalBox<u32, S>) -> u32 {
    signal.set(1);
    *signal.read()
}

fn bench_fib(c: &mut Criterion) {
    {
        let owner = UnsyncStorage::owner();
        c.bench_function("create_unsync", |b| b.iter(|| create(black_box(&owner))));
        let signal = create(&owner);
        c.bench_function("set_read_unsync", |b| {
            b.iter(|| set_read(black_box(signal)))
        });
    }
    {
        let owner = SyncStorage::owner();
        c.bench_function("create_sync", |b| b.iter(|| create(black_box(&owner))));
        let signal = create(&owner);
        c.bench_function("set_read_sync", |b| b.iter(|| set_read(black_box(signal))));
    }
}

criterion_group!(benches, bench_fib);
criterion_main!(benches);
