use {
    bitvec_benchmarking::*,
    criterion::{black_box, criterion_group, criterion_main, BatchSize, BenchmarkId, Criterion},
    rand::prelude::*,
    std::time::Instant,
};

const BIT_WIDTHS: [usize; 5] = [
    64,      // the smallest size we might ever use
    1 << 8,  // max values an 8-bit integer can hold
    1 << 16, // max values a 16-bit integer can hold
    1 << 19, // enough bits to hold an epoch
    1 << 20, // it would be nice to go up to 1<<30, but this is probably the max
];

fn bench_count_ones(c: &mut Criterion) {
    let mut group = c.benchmark_group("count_ones");

    for num_bits in BIT_WIDTHS {
        let data = new_data(num_bits);

        group.bench_function(BenchmarkId::new("bv", num_bits), |bencher| {
            bencher.iter_batched_ref(
                || bv_utils::new_from(data.as_slice()),
                |bv| bv_utils::count_ones(bv),
                BatchSize::LargeInput,
            )
        });

        group.bench_function(BenchmarkId::new("bitvec", num_bits), |bencher| {
            bencher.iter_batched_ref(
                || bitvec_utils::new_from(data.as_slice()),
                |bitvec| bitvec_utils::count_ones(bitvec),
                BatchSize::LargeInput,
            )
        });

        group.bench_function(BenchmarkId::new("bit_vec", num_bits), |bencher| {
            bencher.iter_batched_ref(
                || bit_vec_utils::new_from(data.as_slice()),
                |bit_vec| bit_vec_utils::count_ones(bit_vec),
                BatchSize::LargeInput,
            )
        });

        group.bench_function(BenchmarkId::new("hash_set", num_bits), |bencher| {
            bencher.iter_batched_ref(
                || hash_set_utils::new_from(data.as_slice()),
                |hash_set| hash_set_utils::count_ones(hash_set),
                BatchSize::LargeInput,
            )
        });

        group.bench_function(BenchmarkId::new("int_set", num_bits), |bencher| {
            bencher.iter_batched_ref(
                || int_set_utils::new_from(data.as_slice()),
                |int_set| int_set_utils::count_ones(int_set),
                BatchSize::LargeInput,
            )
        });
    }
}

fn bench_iter_ones(c: &mut Criterion) {
    let mut group = c.benchmark_group("iter_ones");

    for num_bits in BIT_WIDTHS {
        let data = new_data(num_bits);

        group.bench_function(BenchmarkId::new("bv", num_bits), |bencher| {
            bencher.iter_batched_ref(
                || bv_utils::new_from(data.as_slice()),
                |bv| bv_utils::iter_ones(bv),
                BatchSize::LargeInput,
            )
        });

        group.bench_function(BenchmarkId::new("bitvec", num_bits), |bencher| {
            bencher.iter_batched_ref(
                || bitvec_utils::new_from(data.as_slice()),
                |bitvec| bitvec_utils::iter_ones(bitvec),
                BatchSize::LargeInput,
            )
        });

        group.bench_function(BenchmarkId::new("bit_vec", num_bits), |bencher| {
            bencher.iter_batched_ref(
                || bit_vec_utils::new_from(data.as_slice()),
                |bit_vec| bit_vec_utils::iter_ones(bit_vec),
                BatchSize::LargeInput,
            )
        });

        group.bench_function(BenchmarkId::new("hash_set", num_bits), |bencher| {
            bencher.iter_batched_ref(
                || hash_set_utils::new_from(data.as_slice()),
                |hash_set| hash_set_utils::iter_ones(hash_set),
                BatchSize::LargeInput,
            )
        });

        group.bench_function(BenchmarkId::new("int_set", num_bits), |bencher| {
            bencher.iter_batched_ref(
                || int_set_utils::new_from(data.as_slice()),
                |int_set| int_set_utils::iter_ones(int_set),
                BatchSize::LargeInput,
            )
        });
    }
}

fn bench_random_access_load(c: &mut Criterion) {
    let mut group = c.benchmark_group("random_access_load");

    for num_bits in BIT_WIDTHS {
        let data = new_data(num_bits);
        let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(num_bits as u64);

        group.bench_function(BenchmarkId::new("bv", num_bits), |bencher| {
            bencher.iter_custom(|iters| {
                let bv = bv_utils::new_from(data.as_slice());
                std::iter::repeat_with(|| {
                    let bit = rng.gen_range(0..num_bits) as u64;
                    let start = Instant::now();
                    black_box(bv.get(bit));
                    start.elapsed()
                })
                .take(iters as usize)
                .sum()
            });
        });

        group.bench_function(BenchmarkId::new("bitvec", num_bits), |bencher| {
            bencher.iter_custom(|iters| {
                let bitvec = bitvec_utils::new_from(data.as_slice());
                std::iter::repeat_with(|| {
                    let bit = rng.gen_range(0..num_bits);
                    let start = Instant::now();
                    black_box(bitvec.get(bit));
                    start.elapsed()
                })
                .take(iters as usize)
                .sum()
            });
        });

        group.bench_function(BenchmarkId::new("bit_vec", num_bits), |bencher| {
            bencher.iter_custom(|iters| {
                let bit_vec = bit_vec_utils::new_from(data.as_slice());
                std::iter::repeat_with(|| {
                    let bit = rng.gen_range(0..num_bits);
                    let start = Instant::now();
                    black_box(bit_vec.get(bit));
                    start.elapsed()
                })
                .take(iters as usize)
                .sum()
            });
        });

        group.bench_function(BenchmarkId::new("hash_set", num_bits), |bencher| {
            bencher.iter_custom(|iters| {
                let hash_set = hash_set_utils::new_from(data.as_slice());
                std::iter::repeat_with(|| {
                    let bit = rng.gen_range(0..num_bits) as u64;
                    let start = Instant::now();
                    black_box(hash_set.contains(&bit));
                    start.elapsed()
                })
                .take(iters as usize)
                .sum()
            });
        });

        group.bench_function(BenchmarkId::new("int_set", num_bits), |bencher| {
            bencher.iter_custom(|iters| {
                let int_set = int_set_utils::new_from(data.as_slice());
                std::iter::repeat_with(|| {
                    let bit = rng.gen_range(0..num_bits) as u64;
                    let start = Instant::now();
                    black_box(int_set.contains(&bit));
                    start.elapsed()
                })
                .take(iters as usize)
                .sum()
            });
        });
    }
}

fn bench_random_access_set(c: &mut Criterion) {
    let mut group = c.benchmark_group("random_access_set");

    for num_bits in BIT_WIDTHS {
        let data = new_data(num_bits);
        let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(num_bits as u64);

        group.bench_function(BenchmarkId::new("bv", num_bits), |bencher| {
            bencher.iter_custom(|iters| {
                let mut bv = bv_utils::new_from(data.as_slice());
                std::iter::repeat_with(|| {
                    let bit = rng.gen_range(0..num_bits) as u64;
                    let start = Instant::now();
                    bv.set(bit, true);
                    start.elapsed()
                })
                .take(iters as usize)
                .sum()
            });
        });

        group.bench_function(BenchmarkId::new("bitvec", num_bits), |bencher| {
            bencher.iter_custom(|iters| {
                let mut bitvec = bitvec_utils::new_from(data.as_slice());
                std::iter::repeat_with(|| {
                    let bit = rng.gen_range(0..num_bits);
                    let start = Instant::now();
                    bitvec.set(bit, true);
                    start.elapsed()
                })
                .take(iters as usize)
                .sum()
            });
        });

        group.bench_function(BenchmarkId::new("bit_vec", num_bits), |bencher| {
            bencher.iter_custom(|iters| {
                let mut bit_vec = bit_vec_utils::new_from(data.as_slice());
                std::iter::repeat_with(|| {
                    let bit = rng.gen_range(0..num_bits);
                    let start = Instant::now();
                    bit_vec.set(bit, true);
                    start.elapsed()
                })
                .take(iters as usize)
                .sum()
            });
        });

        group.bench_function(BenchmarkId::new("hash_set", num_bits), |bencher| {
            bencher.iter_custom(|iters| {
                let mut hash_set = hash_set_utils::new_from(data.as_slice());
                std::iter::repeat_with(|| {
                    let bit = rng.gen_range(0..num_bits) as u64;
                    let start = Instant::now();
                    black_box(hash_set.insert(bit));
                    start.elapsed()
                })
                .take(iters as usize)
                .sum()
            });
        });

        group.bench_function(BenchmarkId::new("int_set", num_bits), |bencher| {
            bencher.iter_custom(|iters| {
                let mut int_set = int_set_utils::new_from(data.as_slice());
                std::iter::repeat_with(|| {
                    let bit = rng.gen_range(0..num_bits) as u64;
                    let start = Instant::now();
                    black_box(int_set.insert(bit));
                    start.elapsed()
                })
                .take(iters as usize)
                .sum()
            });
        });
    }
}

fn bench_random_access_clear(c: &mut Criterion) {
    let mut group = c.benchmark_group("random_access_clear");

    for num_bits in BIT_WIDTHS {
        let data = new_data(num_bits);
        let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(num_bits as u64);

        group.bench_function(BenchmarkId::new("bv", num_bits), |bencher| {
            bencher.iter_custom(|iters| {
                let mut bv = bv_utils::new_from(data.as_slice());
                std::iter::repeat_with(|| {
                    let bit = rng.gen_range(0..num_bits) as u64;
                    let start = Instant::now();
                    bv.set(bit, false);
                    start.elapsed()
                })
                .take(iters as usize)
                .sum()
            });
        });

        group.bench_function(BenchmarkId::new("bitvec", num_bits), |bencher| {
            bencher.iter_custom(|iters| {
                let mut bitvec = bitvec_utils::new_from(data.as_slice());
                std::iter::repeat_with(|| {
                    let bit = rng.gen_range(0..num_bits);
                    let start = Instant::now();
                    bitvec.set(bit, false);
                    start.elapsed()
                })
                .take(iters as usize)
                .sum()
            });
        });

        group.bench_function(BenchmarkId::new("bit_vec", num_bits), |bencher| {
            bencher.iter_custom(|iters| {
                let mut bit_vec = bit_vec_utils::new_from(data.as_slice());
                std::iter::repeat_with(|| {
                    let bit = rng.gen_range(0..num_bits);
                    let start = Instant::now();
                    bit_vec.set(bit, false);
                    start.elapsed()
                })
                .take(iters as usize)
                .sum()
            });
        });

        group.bench_function(BenchmarkId::new("hash_set", num_bits), |bencher| {
            bencher.iter_custom(|iters| {
                let mut hash_set = hash_set_utils::new_from(data.as_slice());
                std::iter::repeat_with(|| {
                    let bit = rng.gen_range(0..num_bits) as u64;
                    let start = Instant::now();
                    black_box(hash_set.remove(&bit));
                    start.elapsed()
                })
                .take(iters as usize)
                .sum()
            });
        });

        group.bench_function(BenchmarkId::new("int_set", num_bits), |bencher| {
            bencher.iter_custom(|iters| {
                let mut int_set = int_set_utils::new_from(data.as_slice());
                std::iter::repeat_with(|| {
                    let bit = rng.gen_range(0..num_bits) as u64;
                    let start = Instant::now();
                    black_box(int_set.remove(&bit));
                    start.elapsed()
                })
                .take(iters as usize)
                .sum()
            });
        });
    }
}

fn bench_random_access_toggle(c: &mut Criterion) {
    let mut group = c.benchmark_group("random_access_toggle");

    for num_bits in BIT_WIDTHS {
        let data = new_data(num_bits);
        let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(num_bits as u64);

        group.bench_function(BenchmarkId::new("bv", num_bits), |bencher| {
            bencher.iter_custom(|iters| {
                let mut bv = bv_utils::new_from(data.as_slice());
                std::iter::repeat_with(|| {
                    let bit = rng.gen_range(0..num_bits) as u64;
                    let start = Instant::now();
                    let val = bv.get(bit);
                    bv.set(bit, val ^ true);
                    start.elapsed()
                })
                .take(iters as usize)
                .sum()
            });
        });

        group.bench_function(BenchmarkId::new("bitvec", num_bits), |bencher| {
            bencher.iter_custom(|iters| {
                let mut bitvec = bitvec_utils::new_from(data.as_slice());
                std::iter::repeat_with(|| {
                    let bit = rng.gen_range(0..num_bits);
                    let start = Instant::now();
                    let val = *bitvec.get(bit).unwrap();
                    bitvec.set(bit, val ^ true);
                    start.elapsed()
                })
                .take(iters as usize)
                .sum()
            });
        });

        group.bench_function(BenchmarkId::new("bit_vec", num_bits), |bencher| {
            bencher.iter_custom(|iters| {
                let mut bit_vec = bit_vec_utils::new_from(data.as_slice());
                std::iter::repeat_with(|| {
                    let bit = rng.gen_range(0..num_bits);
                    let start = Instant::now();
                    let val = bit_vec.get(bit).unwrap();
                    bit_vec.set(bit, val ^ true);
                    start.elapsed()
                })
                .take(iters as usize)
                .sum()
            });
        });

        group.bench_function(BenchmarkId::new("hash_set", num_bits), |bencher| {
            bencher.iter_custom(|iters| {
                let mut hash_set = hash_set_utils::new_from(data.as_slice());
                std::iter::repeat_with(|| {
                    let bit = rng.gen_range(0..num_bits) as u64;
                    let start = Instant::now();
                    if hash_set.contains(&bit) {
                        black_box(hash_set.remove(&bit));
                    } else {
                        black_box(hash_set.insert(bit));
                    }
                    start.elapsed()
                })
                .take(iters as usize)
                .sum()
            });
        });

        group.bench_function(BenchmarkId::new("int_set", num_bits), |bencher| {
            bencher.iter_custom(|iters| {
                let mut int_set = int_set_utils::new_from(data.as_slice());
                std::iter::repeat_with(|| {
                    let bit = rng.gen_range(0..num_bits) as u64;
                    let start = Instant::now();
                    if int_set.contains(&bit) {
                        black_box(int_set.remove(&bit));
                    } else {
                        black_box(int_set.insert(bit));
                    }
                    start.elapsed()
                })
                .take(iters as usize)
                .sum()
            });
        });
    }
}

criterion_group!(
    benches,
    bench_count_ones,
    bench_iter_ones,
    bench_random_access_load,
    bench_random_access_set,
    bench_random_access_clear,
    bench_random_access_toggle,
);
criterion_main!(benches);
