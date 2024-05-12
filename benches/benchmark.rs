use criterion::{black_box, criterion_group, criterion_main, Criterion};
use hashbrown::HashMap as HashbrownHashMap;
use rand::distributions::{Alphanumeric, DistString};
use std::collections::HashMap;
extern crate fxhash;
use fnv::FnvHashMap;
use fxhash::FxHashMap;

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref SMALL_STRING: String = Alphanumeric.sample_string(&mut rand::thread_rng(), 8);
    static ref LARGE_STRING: String = Alphanumeric.sample_string(&mut rand::thread_rng(), 256);
}
fn build_std_hashmap_with_key<T>(capacity: usize) -> HashMap<T, u8> {
    HashMap::with_capacity(capacity)
}

fn build_hashbrown_hashmap_with_key<T>(capacity: usize) -> HashbrownHashMap<T, u8> {
    HashbrownHashMap::with_capacity(capacity)
}

fn build_fxhash_hashmap_with_key<T>(capacity: usize) -> FxHashMap<T, u8> {
    FxHashMap::with_capacity_and_hasher(capacity, Default::default())
}

fn build_fnv_hashmap_with_key<T>(capacity: usize) -> FnvHashMap<T, u8> {
    FnvHashMap::with_capacity_and_hasher(capacity, Default::default())
}

pub fn hashmap_benchmark_for_u8(c: &mut Criterion) {
    let mut group = c.benchmark_group("hashmap insert with u8 key");
    let mut std_hm = build_std_hashmap_with_key::<u8>(4);
    let mut hashbrown_hm = build_hashbrown_hashmap_with_key::<u8>(4);
    let mut fxhash_hm = build_fxhash_hashmap_with_key::<u8>(4);
    let mut fnv_hm = build_fnv_hashmap_with_key::<u8>(4);

    group.sample_size(500);
    group.bench_function("std", |b| {
        b.iter(|| {
            std_hm.insert(1u8, 1);
        });
    });
    group.bench_function("hashbrown", |b| {
        b.iter(|| {
            hashbrown_hm.insert(1u8, 1);
        });
    });
    group.bench_function("fxhash", |b| {
        b.iter(|| {
            fxhash_hm.insert(1u8, 1);
        });
    });
    group.bench_function("fnv", |b| {
        b.iter(|| {
            fnv_hm.insert(1u8, 1);
        });
    });
    group.finish();
}

pub fn hashmap_benchmark_for_u64(c: &mut Criterion) {
    let mut group = c.benchmark_group("hashmap insert with u64 key");
    let mut std_hm = build_std_hashmap_with_key::<u64>(4);
    let mut hashbrown_hm = build_hashbrown_hashmap_with_key::<u64>(4);
    let mut fxhash_hm = build_fxhash_hashmap_with_key::<u64>(4);
    let mut fnv_hm = build_fnv_hashmap_with_key::<u64>(4);
    group.sample_size(500);
    group.bench_function("std", |b| {
        b.iter(|| {
            std_hm.insert(u64::MAX, 1);
        });
    });
    group.bench_function("hashbrown", |b| {
        b.iter(|| {
            hashbrown_hm.insert(u64::MAX, 1);
        });
    });
    group.bench_function("fxhash", |b| {
        b.iter(|| {
            fxhash_hm.insert(u64::MAX, 1);
        });
    });
    group.bench_function("fnv", |b| {
        b.iter(|| {
            fnv_hm.insert(u64::MAX, 1);
        });
    });
    group.finish();
}

pub fn hashmap_benchmark_for_small_string(c: &mut Criterion) {
    let mut group = c.benchmark_group("hashmap insert with small string (8 Byte)");
    let mut std_hm = build_std_hashmap_with_key::<&str>(4);
    let mut hashbrown_hm = build_hashbrown_hashmap_with_key::<&str>(4);
    let mut fxhash_hm = build_fxhash_hashmap_with_key::<&str>(4);
    let mut fnv_hm = build_fnv_hashmap_with_key::<&str>(4);
    group.sample_size(500);
    group.bench_function("std", |b| {
        b.iter(|| {
            std_hm.insert(&SMALL_STRING, 1);
        });
    });
    group.bench_function("hashbrown", |b| {
        b.iter(|| {
            hashbrown_hm.insert(&SMALL_STRING, 1);
        });
    });
    group.bench_function("fxhash", |b| {
        b.iter(|| {
            fxhash_hm.insert(&SMALL_STRING, 1);
        });
    });
    group.bench_function("fnv", |b| {
        b.iter(|| {
            fnv_hm.insert(&SMALL_STRING, 1);
        });
    });
    group.finish();
}

pub fn hashmap_benchmark_for_large_string(c: &mut Criterion) {
    let mut group = c.benchmark_group("hashmap insert with large string (256 Byte)");
    let mut std_hm = build_std_hashmap_with_key::<&str>(4);
    let mut hashbrown_hm = build_hashbrown_hashmap_with_key::<&str>(4);
    let mut fxhash_hm = build_fxhash_hashmap_with_key::<&str>(4);
    let mut fnv_hm = build_fnv_hashmap_with_key::<&str>(4);
    group.sample_size(500);
    group.bench_function("std", |b| {
        b.iter(|| {
            std_hm.insert(&LARGE_STRING, 1);
        });
    });
    group.bench_function("hashbrown", |b| {
        b.iter(|| {
            hashbrown_hm.insert(&LARGE_STRING, 1);
        });
    });
    group.bench_function("fxhash", |b| {
        b.iter(|| {
            fxhash_hm.insert(&LARGE_STRING, 1);
        });
    });
    group.bench_function("fnv", |b| {
        b.iter(|| {
            fnv_hm.insert(&LARGE_STRING, 1);
        });
    });
    group.finish();
}

pub fn hashmap_benchmark_for_iteration(c: &mut Criterion) {
    let mut group = c.benchmark_group("hashmap iteration");
    let mut std_hm = build_std_hashmap_with_key::<&str>(4);
    let mut hashbrown_hm = build_hashbrown_hashmap_with_key::<&str>(4);
    let mut fxhash_hm = build_fxhash_hashmap_with_key::<&str>(4);
    let mut fnv_hm = build_fnv_hashmap_with_key::<&str>(4);
    std_hm.insert(&SMALL_STRING, 1);
    hashbrown_hm.insert(&SMALL_STRING, 1);
    fxhash_hm.insert(&SMALL_STRING, 1);
    fnv_hm.insert(&SMALL_STRING, 1);
    group.sample_size(500);
    group.bench_function("std", |b| {
        b.iter(|| {
            for (key, value) in std_hm.iter() {
                black_box(key);
                black_box(value);
            }
        });
    });
    group.bench_function("hashbrown", |b| {
        b.iter(|| {
            for (key, value) in hashbrown_hm.iter() {
                black_box(key);
                black_box(value);
            }
        });
    });
    group.bench_function("fxhash", |b| {
        b.iter(|| {
            for (key, value) in fxhash_hm.iter() {
                black_box(key);
                black_box(value);
            }
        });
    });
    group.bench_function("fnv", |b| {
        b.iter(|| {
            for (key, value) in fnv_hm.iter() {
                black_box(key);
                black_box(value);
            }
        });
    });
    group.finish();
}

pub fn hashmap_benchmark_for_u64_lookup(c: &mut Criterion) {
    const SIZE: usize = 1_000;
    let mut group = c.benchmark_group("hashmap lookup with u64 key");
    let mut std_hm = build_std_hashmap_with_key::<u64>(SIZE);
    let mut hashbrown_hm = build_hashbrown_hashmap_with_key::<u64>(SIZE);
    let mut fxhash_hm = build_fxhash_hashmap_with_key::<u64>(SIZE);
    let mut fnv_hm = build_fnv_hashmap_with_key::<u64>(SIZE);
    for i in 0..SIZE as u64 {
        std_hm.insert(i, 1);
        hashbrown_hm.insert(i, 1);
        fxhash_hm.insert(i, 1);
        fnv_hm.insert(i, 1);
    }
    group.sample_size(500);
    let lookup = (SIZE / 2) as u64;
    group.bench_function("std", |b| {
        b.iter(|| {
            black_box(std_hm.get(&lookup));
        });
    });
    group.bench_function("hashbrown", |b| {
        b.iter(|| {
            black_box(hashbrown_hm.get(&lookup));
        });
    });
    group.bench_function("fxhash", |b| {
        b.iter(|| {
            black_box(fxhash_hm.get(&lookup));
        });
    });
    group.bench_function("fnv", |b| {
        b.iter(|| {
            black_box(fnv_hm.get(&lookup));
        });
    });
    group.finish();
}

pub fn custom_key_value_store_benchmark_for_u64_insert(c: &mut Criterion) {
    //let mut b_kv = key_value_store::KeyValueStore::<u64, u8>::with_capacity(4);
    let mut group = c.benchmark_group("key value store insert for u64");
    group.sample_size(500);
    group.bench_function("key value store insert for u64", |b| {
        let mut b_kv = key_value_store::KeyValueStore::<_, _, 1>::new();
        b.iter(|| b_kv.insert(black_box(1u64), 0u8));
    });
    group.finish();
}

pub fn custom_key_value_store_benchmark_for_u64_lookup(c: &mut Criterion) {
    let mut group = c.benchmark_group("key value store lookup for u64");

    let mut b_kv = key_value_store::KeyValueStore::<_, _, 1>::new();

    const SIZE: u64 = 10;

    for i in 0..SIZE {
        b_kv.insert(i, 1);
    }
    group.sample_size(500);
    group.bench_function("best scenario", |b| {
        b.iter(|| {
            black_box(b_kv.get(&0));
        });
    });

    group.bench_function("worst scenario", |b| {
        let lookup = SIZE - 1;
        b.iter(|| {
            black_box(b_kv.get(&lookup));
        });
    });

    group.finish();
}

pub fn custom_key_value_store_benchmark_for_string_lookup(c: &mut Criterion) {
    let mut group = c.benchmark_group("key value store lookup for string");

    let mut b_kv = key_value_store::KeyValueStore::<_, _, 10>::new();

    const SIZE: u64 = 10;

    let first_string = Alphanumeric.sample_string(&mut rand::thread_rng(), 8);
    let last_string = Alphanumeric.sample_string(&mut rand::thread_rng(), 8);
    for i in 0..SIZE {
        match i {
            0 => b_kv.insert(first_string.clone(), 1),
            9 => b_kv.insert(last_string.clone(), 1),
            _ => b_kv.insert(Alphanumeric.sample_string(&mut rand::thread_rng(), 8), 1),
        }
    }
    group.sample_size(500);
    group.bench_function("best scenario", |b| {
        b.iter(|| {
            black_box(b_kv.get(&first_string));
        });
    });

    group.bench_function("worst scenario", |b| {
        b.iter(|| {
            black_box(b_kv.get(&last_string));
        });
    });

    group.finish();
}

pub fn custom_key_value_store_benchmark_for_iteration(c: &mut Criterion) {
    let mut group = c.benchmark_group("key value store iteration");

    let mut b_kv = key_value_store::KeyValueStore::<_, _, 10>::new();

    b_kv.insert(u64::MAX, 1);

    group.sample_size(500);
    group.bench_function("key value store iteration", |b| {
        b.iter(|| {
            for (key, value) in b_kv.items() {
                black_box(key);
                black_box(value);
            }
        });
    });

    group.finish();
}

pub fn custom_key_value_store_benchmark_for_u64_lookup_by_index(c: &mut Criterion) {
    let mut group = c.benchmark_group("key value store lookup for u64");

    let mut b_kv = key_value_store::KeyValueStore::<_, _, 10>::new();

    const SIZE: u64 = 10;

    for i in 0..SIZE {
        b_kv.insert(i, 1);
    }
    group.sample_size(500);
    group.bench_function("best scenario", |b| {
        b.iter(|| {
            black_box(b_kv.get_by_index(0));
        });
    });

    group.bench_function("worst scenario", |b| {
        let lookup = (SIZE - 1) as usize;
        b.iter(|| {
            black_box(b_kv.get_by_index(lookup));
        });
    });

    group.finish();
}

criterion_group!(
    benches,
    custom_key_value_store_benchmark_for_u64_lookup,
    custom_key_value_store_benchmark_for_u64_lookup_by_index,
    custom_key_value_store_benchmark_for_string_lookup,
    custom_key_value_store_benchmark_for_iteration
);
criterion_group!(benches2, custom_key_value_store_benchmark_for_u64_insert,);
criterion_main!(benches, benches2);
