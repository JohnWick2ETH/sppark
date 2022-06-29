// Copyright Supranational LLC
// Licensed under the Apache License, Version 2.0, see LICENSE for details.
// SPDX-License-Identifier: Apache-2.0

use criterion::{criterion_group, criterion_main, Criterion};

#[cfg(feature = "bls12_377")]
use ark_bls12_377::G1Affine;
#[cfg(feature = "bls12_381")]
use ark_bls12_381::G1Affine;
#[cfg(feature = "bn254")]
use ark_bn254::G1Affine;
use ark_ff::BigInteger256;

use std::str::FromStr;

use blst_msm::*;

fn criterion_benchmark(c: &mut Criterion) {
    let bench_npow = std::env::var("BENCH_NPOW").unwrap_or("23".to_string());
    let npoints_npow = i32::from_str(&bench_npow).unwrap();

    let (points, scalars) =
        util::generate_points_scalars::<G1Affine>(1usize << npoints_npow);

    let mut group = c.benchmark_group("CUDA");
    group.sample_size(20);

    for i in 18..=npoints_npow {
        let name = format!("2**{}", i);
        group.bench_function(name, |b| {
            b.iter(|| {
                let _ = multi_scalar_mult_arkworks(&points[..(1<<i)], unsafe {
                    std::mem::transmute::<&[_], &[BigInteger256]>(
                        &scalars[..(1<<i)],
                    )
                });
            })
        });
    }

    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
