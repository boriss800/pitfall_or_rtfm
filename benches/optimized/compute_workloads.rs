use std::fs::File;
use std::io::{self, BufRead, BufReader};
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use wide::f64x4;

/// Optimized Jaro-Winkler with byte-level processing and efficient memory usage
fn optimized_jaro_winkler(s1: &str, s2: &str) -> f64 {
    let s1_bytes = s1.as_bytes();
    let s2_bytes = s2.as_bytes();
    
    let len1 = s1_bytes.len();
    let len2 = s2_bytes.len();
    
    if len1 == 0 && len2 == 0 {
        return 1.0;
    }
    if len1 == 0 || len2 == 0 {
        return 0.0;
    }
    
    let match_window = std::cmp::max(len1, len2) / 2;
    if match_window == 0 {
        return if s1 == s2 { 1.0 } else { 0.0 };
    }
    
    // Use bit vectors for matches (more memory efficient)
    let mut s1_matches = vec![false; len1];
    let mut s2_matches = vec![false; len2];
    
    let mut matches = 0;
    
    // Optimized matching with early termination
    for (i, &c1) in s1_bytes.iter().enumerate() {
        let start = i.saturating_sub(match_window);
        let end = std::cmp::min(i + match_window + 1, len2);
        
        for j in start..end {
            if !s2_matches[j] && s2_bytes[j] == c1 {
                s1_matches[i] = true;
                s2_matches[j] = true;
                matches += 1;
                break;
            }
        }
    }
    
    if matches == 0 {
        return 0.0;
    }
    
    // Efficient transposition counting
    let mut transpositions = 0;
    let mut k = 0;
    for i in 0..len1 {
        if !s1_matches[i] {
            continue;
        }
        while !s2_matches[k] {
            k += 1;
        }
        if s1_bytes[i] != s2_bytes[k] {
            transpositions += 1;
        }
        k += 1;
    }
    
    let jaro = (matches as f64 / len1 as f64 + 
                matches as f64 / len2 as f64 + 
                (matches as f64 - transpositions as f64 / 2.0) / matches as f64) / 3.0;
    
    // Optimized prefix calculation
    let prefix_len = s1_bytes.iter()
        .zip(s2_bytes.iter())
        .take(4)
        .take_while(|(a, b)| a == b)
        .count();
    
    jaro + 0.1 * prefix_len as f64 * (1.0 - jaro)
}

/// Optimized string similarity processing with pre-allocation
fn optimized_string_similarity() -> io::Result<Vec<f64>> {
    let file = File::open("/tmp/benchmark_data/string_pairs.txt")?;
    let reader = BufReader::with_capacity(64 * 1024, file);
    
    let mut similarities = Vec::with_capacity(1_000_000); // Pre-allocate
    
    for line in reader.lines() {
        let line = line?;
        
        // Use split_once for better performance
        if let Some((s1, s2)) = line.split_once('\t') {
            let similarity = optimized_jaro_winkler(s1, s2);
            similarities.push(similarity);
        }
    }
    
    Ok(similarities)
}

/// SIMD-optimized numeric aggregation
fn optimized_numeric_aggregation() -> io::Result<(f64, f64, f64)> {
    let data = std::fs::read("/tmp/benchmark_data/numeric_data.bin")?;
    
    // Convert bytes to f64 values efficiently using unsafe (zero-copy)
    let values: &[f64] = unsafe {
        std::slice::from_raw_parts(
            data.as_ptr() as *const f64,
            data.len() / 8
        )
    };
    
    // SIMD-optimized sum calculation
    let chunks = values.chunks_exact(4);
    let remainder = chunks.remainder();
    
    let simd_sum: f64x4 = values
        .chunks_exact(4)
        .map(|chunk| f64x4::from([chunk[0], chunk[1], chunk[2], chunk[3]]))
        .fold(f64x4::ZERO, |acc, x| acc + x);
    
    let sum = simd_sum.reduce_add() + remainder.iter().sum::<f64>();
    let mean = sum / values.len() as f64;
    
    // SIMD-optimized variance calculation
    let mean_vec = f64x4::splat(mean);
    let variance_sum: f64x4 = values
        .chunks_exact(4)
        .map(|chunk| {
            let vals = f64x4::from([chunk[0], chunk[1], chunk[2], chunk[3]]);
            let diff = vals - mean_vec;
            diff * diff
        })
        .fold(f64x4::ZERO, |acc, x| acc + x);
    
    let variance = variance_sum.reduce_add() + 
                   remainder.iter().map(|&x| (x - mean).powi(2)).sum::<f64>();
    let std_dev = (variance / values.len() as f64).sqrt();
    
    // Efficient percentile calculation using select_nth_unstable_by
    let mut values_copy = values.to_vec();
    let p95_index = (0.95 * values_copy.len() as f64) as usize;
    values_copy.select_nth_unstable_by(p95_index, |a, b| a.partial_cmp(b).unwrap());
    let p95 = values_copy[p95_index];
    
    Ok((mean, std_dev, p95))
}

/// Optimized prime calculation using Sieve of Eratosthenes
fn optimized_prime_calculation() -> Vec<usize> {
    let limit = 100_000;
    let mut is_prime = vec![true; limit + 1];
    is_prime[0] = false;
    is_prime[1] = false;
    
    // Sieve of Eratosthenes - much more efficient than trial division
    for i in 2..=((limit as f64).sqrt() as usize) {
        if is_prime[i] {
            for j in ((i * i)..=limit).step_by(i) {
                is_prime[j] = false;
            }
        }
    }
    
    // Collect primes efficiently
    is_prime.iter()
        .enumerate()
        .filter_map(|(i, &prime)| if prime { Some(i) } else { None })
        .collect()
}

/// Optimized matrix operations with cache-friendly access and SIMD
fn optimized_matrix_operations() -> Vec<Vec<f64>> {
    let size = 500;
    
    // Pre-allocate matrices with proper capacity
    let matrix_a: Vec<Vec<f64>> = (0..size)
        .map(|i| {
            (0..size).map(|j| (i * j) as f64).collect()
        })
        .collect();
    
    let matrix_b: Vec<Vec<f64>> = (0..size)
        .map(|i| {
            (0..size).map(|j| (i + j) as f64).collect()
        })
        .collect();
    
    // Cache-friendly blocked matrix multiplication
    let mut result = vec![vec![0.0; size]; size];
    const BLOCK_SIZE: usize = 64;
    
    for ii in (0..size).step_by(BLOCK_SIZE) {
        for jj in (0..size).step_by(BLOCK_SIZE) {
            for kk in (0..size).step_by(BLOCK_SIZE) {
                let i_end = std::cmp::min(ii + BLOCK_SIZE, size);
                let j_end = std::cmp::min(jj + BLOCK_SIZE, size);
                let k_end = std::cmp::min(kk + BLOCK_SIZE, size);
                
                for i in ii..i_end {
                    for k in kk..k_end {
                        let a_ik = matrix_a[i][k];
                        for j in jj..j_end {
                            result[i][j] += a_ik * matrix_b[k][j];
                        }
                    }
                }
            }
        }
    }
    
    result
}

/// Optimized fibonacci using dynamic programming (memoization)
fn optimized_fibonacci(n: u32) -> u64 {
    if n <= 1 {
        return n as u64;
    }
    
    let mut fib = vec![0u64; (n + 1) as usize];
    fib[1] = 1;
    
    for i in 2..=n as usize {
        fib[i] = fib[i - 1] + fib[i - 2];
    }
    
    fib[n as usize]
}

/// Optimized sorting using Rust's highly optimized sort algorithms
fn optimized_sorting() -> Vec<i32> {
    // Generate test data
    let mut data: Vec<i32> = (0..10_000).rev().collect();
    
    // Use Rust's optimized unstable sort (typically introsort)
    data.sort_unstable();
    
    data
}

/// Optimized hash computation using SIMD and efficient algorithms
fn optimized_hash_computation() -> Vec<u64> {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    
    let data: Vec<String> = (0..100_000)
        .map(|i| format!("item_{}", i))
        .collect();
    
    let mut hashes = Vec::with_capacity(data.len());
    
    for item in &data {
        let mut hasher = DefaultHasher::new();
        item.hash(&mut hasher);
        hashes.push(hasher.finish());
    }
    
    hashes
}

/// Optimized pattern matching using efficient algorithms
fn optimized_pattern_matching() -> io::Result<Vec<usize>> {
    let file = File::open("/tmp/benchmark_data/text_corpus.txt")?;
    let reader = BufReader::with_capacity(64 * 1024, file);
    
    let pattern = "optimization";
    let mut matches = Vec::new();
    
    for (line_num, line) in reader.lines().enumerate() {
        let line = line?;
        
        // Use efficient string searching (Boyer-Moore-like algorithm in std)
        if let Some(pos) = line.find(pattern) {
            matches.push(line_num * 1000 + pos); // Encode line and position
        }
    }
    
    Ok(matches)
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("optimized_string_similarity", |b| {
        b.iter(|| black_box(optimized_string_similarity().unwrap()))
    });
    
    c.bench_function("optimized_numeric_aggregation", |b| {
        b.iter(|| black_box(optimized_numeric_aggregation().unwrap()))
    });
    
    c.bench_function("optimized_prime_calculation", |b| {
        b.iter(|| black_box(optimized_prime_calculation()))
    });
    
    c.bench_function("optimized_matrix_operations", |b| {
        b.iter(|| black_box(optimized_matrix_operations()))
    });
    
    c.bench_function("optimized_fibonacci", |b| {
        b.iter(|| black_box(optimized_fibonacci(35)))
    });
    
    c.bench_function("optimized_sorting", |b| {
        b.iter(|| black_box(optimized_sorting()))
    });
    
    c.bench_function("optimized_hash_computation", |b| {
        b.iter(|| black_box(optimized_hash_computation()))
    });
    
    c.bench_function("optimized_pattern_matching", |b| {
        b.iter(|| black_box(optimized_pattern_matching().unwrap()))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
