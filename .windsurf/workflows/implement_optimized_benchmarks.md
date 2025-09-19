# Implement Optimized Benchmarks

**Keywords:** optimized, performance, simd, rayon, buffered, release, efficient
**Flags:** `day-3`, `optimized-implementation`, `performance-critical`

## Description
Implement all optimized benchmark implementations using Rust best practices, SIMD, parallelism, and memory-efficient techniques. These demonstrate the full potential of optimized Rust code.

## Critical Requirements
- **ALWAYS use release mode** with full optimizations
- **Apply all relevant optimizations** from the PRD
- **Use buffered I/O** for all file operations
- **Leverage SIMD** where applicable
- **Use Rayon** for parallel workloads
- **Pre-allocate memory** with known capacities
- **Avoid unnecessary clones** and allocations

## Steps

### 1. Implement Optimized I/O Benchmarks
Create `benches/optimized/io_workloads.rs`:

```rust
use std::fs::File;
use std::io::{self, BufRead, BufReader, BufWriter, Write};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

// Optimized file reading with buffering and streaming
fn optimized_large_file_processing() -> io::Result<usize> {
    let file = File::open("/tmp/benchmark_data/large_text.txt")?;
    let reader = BufReader::with_capacity(64 * 1024, file);
    
    let word_count = reader
        .lines()
        .map(|line| {
            line.unwrap()
                .split_whitespace()
                .count()  // No intermediate collection
        })
        .sum();
    
    Ok(word_count)
}

// Optimized CSV transformation with buffered I/O and pre-allocation
fn optimized_csv_transformation() -> io::Result<()> {
    let file = File::open("/tmp/benchmark_data/large_data.csv")?;
    let reader = BufReader::with_capacity(64 * 1024, file);
    
    let output_file = File::create("/tmp/benchmark_data/output_optimized.csv")?;
    let mut writer = BufWriter::with_capacity(64 * 1024, output_file);
    
    for line in reader.lines() {
        let line = line?;
        let mut output_line = String::with_capacity(line.len() * 2); // Pre-allocate
        
        for (i, field) in line.split(',').enumerate() {
            if i > 0 {
                output_line.push(',');
            }
            // Efficient uppercase conversion
            for ch in field.chars() {
                output_line.extend(ch.to_uppercase());
            }
        }
        output_line.push('\n');
        
        writer.write_all(output_line.as_bytes())?;
    }
    
    writer.flush()?;
    Ok(())
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("optimized_large_file_processing", |b| {
        b.iter(|| black_box(optimized_large_file_processing().unwrap()))
    });
    
    c.bench_function("optimized_csv_transformation", |b| {
        b.iter(|| black_box(optimized_csv_transformation().unwrap()))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
```

### 2. Implement Optimized Parsing Benchmarks
Create `benches/optimized/parsing_workloads.rs`:

```rust
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::collections::HashMap;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use serde_json;
use ahash::AHashMap; // Faster hash map

// Optimized text tokenization with zero-copy and efficient hashing
fn optimized_text_tokenization() -> io::Result<AHashMap<String, usize>> {
    let file = File::open("/tmp/benchmark_data/text_corpus.txt")?;
    let reader = BufReader::with_capacity(64 * 1024, file);
    
    let mut word_counts = AHashMap::with_capacity(100_000); // Pre-allocate
    
    for line in reader.lines() {
        let line = line?;
        
        // Use iterator chain without intermediate collections
        for word in line
            .split_whitespace()
            .map(|w| w.to_lowercase()) // No clone needed
        {
            *word_counts.entry(word).or_insert(0) += 1;
        }
    }
    
    Ok(word_counts)
}

// Optimized JSON parsing with streaming and zero-copy where possible
fn optimized_json_processing() -> io::Result<Vec<serde_json::Value>> {
    let file = File::open("/tmp/benchmark_data/json_records.jsonl")?;
    let reader = BufReader::with_capacity(64 * 1024, file);
    
    let mut records = Vec::with_capacity(10_000_000); // Pre-allocate based on expected size
    
    for line in reader.lines() {
        let line = line?;
        
        // Parse directly from &str without additional string allocation
        if let Ok(value) = serde_json::from_str::<serde_json::Value>(&line) {
            records.push(value); // No clone needed
        }
    }
    
    records.shrink_to_fit(); // Optimize memory usage
    Ok(records)
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("optimized_text_tokenization", |b| {
        b.iter(|| black_box(optimized_text_tokenization().unwrap()))
    });
    
    c.bench_function("optimized_json_processing", |b| {
        b.iter(|| black_box(optimized_json_processing().unwrap()))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
```

### 3. Implement Optimized Computational Benchmarks
Create `benches/optimized/compute_workloads.rs`:

```rust
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use wide::f64x4; // SIMD operations

// Optimized Jaro-Winkler with SIMD and efficient memory usage
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

// Optimized string similarity processing with pre-allocation
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

// SIMD-optimized numeric aggregation
fn optimized_numeric_aggregation() -> io::Result<(f64, f64, f64)> {
    let data = std::fs::read("/tmp/benchmark_data/numeric_data.bin")?;
    
    // Convert bytes to f64 values efficiently
    let values: &[f64] = unsafe {
        std::slice::from_raw_parts(
            data.as_ptr() as *const f64,
            data.len() / 8
        )
    };
    
    // SIMD-optimized sum calculation
    let chunks = values.chunks_exact(4);
    let remainder = chunks.remainder();
    
    let simd_sum: f64x4 = chunks
        .map(|chunk| f64x4::from([chunk[0], chunk[1], chunk[2], chunk[3]]))
        .fold(f64x4::ZERO, |acc, x| acc + x);
    
    let sum = simd_sum.reduce_add() + remainder.iter().sum::<f64>();
    let mean = sum / values.len() as f64;
    
    // SIMD-optimized variance calculation
    let mean_vec = f64x4::splat(mean);
    let variance_sum: f64x4 = chunks
        .map(|chunk| {
            let vals = f64x4::from([chunk[0], chunk[1], chunk[2], chunk[3]]);
            let diff = vals - mean_vec;
            diff * diff
        })
        .fold(f64x4::ZERO, |acc, x| acc + x);
    
    let variance = variance_sum.reduce_add() + 
                   remainder.iter().map(|&x| (x - mean).powi(2)).sum::<f64>();
    let std_dev = (variance / values.len() as f64).sqrt();
    
    // Efficient percentile calculation using select_nth_unstable
    let mut values_copy = values.to_vec();
    let p95_index = (0.95 * values_copy.len() as f64) as usize;
    values_copy.select_nth_unstable(p95_index);
    let p95 = values_copy[p95_index];
    
    Ok((mean, std_dev, p95))
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("optimized_string_similarity", |b| {
        b.iter(|| black_box(optimized_string_similarity().unwrap()))
    });
    
    c.bench_function("optimized_numeric_aggregation", |b| {
        b.iter(|| black_box(optimized_numeric_aggregation().unwrap()))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
```

### 4. Implement Optimized Parallel Benchmarks
Create `benches/optimized/parallel_workloads.rs`:

```rust
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::collections::HashMap;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rayon::prelude::*;
use ahash::AHashMap;

// Optimized parallel word count using Rayon
fn optimized_word_count() -> io::Result<AHashMap<String, usize>> {
    let file_paths = [
        "/tmp/benchmark_data/large_text.txt",
        "/tmp/benchmark_data/text_corpus.txt",
    ];
    
    // Process files in parallel and collect results
    let results: Result<Vec<_>, io::Error> = file_paths
        .par_iter()
        .map(|path| {
            let file = File::open(path)?;
            let reader = BufReader::with_capacity(64 * 1024, file);
            
            let mut local_counts = AHashMap::with_capacity(50_000);
            
            for line in reader.lines() {
                let line = line?;
                
                // Process words in parallel within each line for large lines
                if line.len() > 1000 {
                    let words: Vec<String> = line
                        .par_split_whitespace()
                        .map(|w| w.to_lowercase())
                        .collect();
                    
                    for word in words {
                        *local_counts.entry(word).or_insert(0) += 1;
                    }
                } else {
                    // Sequential processing for small lines (avoid overhead)
                    for word in line.split_whitespace().map(|w| w.to_lowercase()) {
                        *local_counts.entry(word).or_insert(0) += 1;
                    }
                }
            }
            
            Ok(local_counts)
        })
        .collect();
    
    // Merge results from all files
    let file_results = results?;
    let mut global_counts = AHashMap::with_capacity(100_000);
    
    for local_counts in file_results {
        for (word, count) in local_counts {
            *global_counts.entry(word).or_insert(0) += count;
        }
    }
    
    Ok(global_counts)
}

// Optimized parallel matrix multiplication with cache-friendly access
fn optimized_matrix_multiplication() -> Vec<Vec<f64>> {
    let size = 1000;
    
    // Pre-allocate matrices with proper capacity
    let matrix_a: Vec<Vec<f64>> = (0..size)
        .into_par_iter()
        .map(|i| {
            (0..size).map(|j| (i * j) as f64).collect()
        })
        .collect();
    
    let matrix_b: Vec<Vec<f64>> = (0..size)
        .into_par_iter()
        .map(|i| {
            (0..size).map(|j| (i + j) as f64).collect()
        })
        .collect();
    
    // Parallel matrix multiplication with cache-friendly access pattern
    let result: Vec<Vec<f64>> = (0..size)
        .into_par_iter()
        .map(|i| {
            let mut row = vec![0.0; size];
            
            // Cache-friendly blocked multiplication
            const BLOCK_SIZE: usize = 64;
            
            for jj in (0..size).step_by(BLOCK_SIZE) {
                for kk in (0..size).step_by(BLOCK_SIZE) {
                    let j_end = std::cmp::min(jj + BLOCK_SIZE, size);
                    let k_end = std::cmp::min(kk + BLOCK_SIZE, size);
                    
                    for k in kk..k_end {
                        let a_ik = matrix_a[i][k];
                        for j in jj..j_end {
                            row[j] += a_ik * matrix_b[k][j];
                        }
                    }
                }
            }
            
            row
        })
        .collect();
    
    result
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("optimized_word_count", |b| {
        b.iter(|| black_box(optimized_word_count().unwrap()))
    });
    
    c.bench_function("optimized_matrix_multiplication", |b| {
        b.iter(|| black_box(optimized_matrix_multiplication()))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
```

### 5. Implement Optimized Memory Benchmarks
Create `benches/optimized/memory_workloads.rs`:

```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rayon::prelude::*;

// Optimized collection pipeline with streaming iterators
fn optimized_collection_pipeline() -> Vec<i32> {
    (0..10_000_000)
        .into_par_iter()  // Parallel processing
        .filter(|&x| x % 2 == 0)
        .map(|x| x * 2)
        .filter(|&x| x > 1000)
        .collect()  // Single collection at the end
}

// Optimized string building with pre-allocation
fn optimized_string_building() -> String {
    let capacity = 100_000 * 20; // Estimate final size
    let mut result = String::with_capacity(capacity);
    
    for i in 0..100_000 {
        // Use format_args! for more efficient formatting
        use std::fmt::Write;
        write!(result, "Item {}: {}\n", i, i * i).unwrap();
    }
    
    result.shrink_to_fit(); // Optimize final memory usage
    result
}

// Optimized vector operations with proper pre-allocation
fn optimized_vector_operations() -> Vec<Vec<i32>> {
    let outer_size = 10_000;
    let inner_size = 100;
    
    // Pre-allocate outer vector
    let mut result = Vec::with_capacity(outer_size);
    
    // Use parallel processing for independent operations
    let data: Vec<Vec<i32>> = (0..outer_size)
        .into_par_iter()
        .map(|i| {
            // Pre-allocate inner vector
            let mut inner = Vec::with_capacity(inner_size);
            
            // Efficient computation without reallocations
            for j in 0..inner_size {
                inner.push(i * j);
            }
            
            inner
        })
        .collect();
    
    data
}

// Optimized streaming operations without intermediate collections
fn optimized_streaming_operations() -> i64 {
    (0..10_000_000i64)
        .into_par_iter()
        .filter(|&x| x % 3 == 0)
        .map(|x| x * x)
        .filter(|&x| x % 7 == 0)
        .map(|x| x / 2)
        .sum()  // Direct reduction without intermediate storage
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("optimized_collection_pipeline", |b| {
        b.iter(|| black_box(optimized_collection_pipeline()))
    });
    
    c.bench_function("optimized_string_building", |b| {
        b.iter(|| black_box(optimized_string_building()))
    });
    
    c.bench_function("optimized_vector_operations", |b| {
        b.iter(|| black_box(optimized_vector_operations()))
    });
    
    c.bench_function("optimized_streaming_operations", |b| {
        b.iter(|| black_box(optimized_streaming_operations()))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
```

## Validation Checklist
- [ ] All benchmarks use release builds with full optimizations
- [ ] Buffered I/O is used for all file operations
- [ ] SIMD operations are applied where beneficial
- [ ] Rayon parallelism is used for CPU-bound operations
- [ ] Memory is pre-allocated with appropriate capacities
- [ ] Iterator chains avoid intermediate collections
- [ ] Zero-copy techniques are used where possible
- [ ] Cache-friendly access patterns are implemented
- [ ] All benchmarks use `criterion::black_box`
- [ ] Error handling is efficient and appropriate

## Performance Expectations
Based on PRD targets, expect these improvements over baseline:
- **I/O workloads**: 3-5x improvement (buffering + streaming)
- **Parsing workloads**: 2-4x improvement (zero-copy + efficient hashing)
- **Computational workloads**: 4-8x improvement (SIMD + algorithms)
- **Parallel workloads**: 4-8x improvement (multi-core utilization)
- **Memory workloads**: 2-10x improvement (allocation reduction)

## Next Steps
After completing optimized implementations:
1. Run `/validate-benchmark-correctness` to ensure both versions produce same results
2. Run `/execute-comprehensive-benchmarks` to get full performance comparison
3. Run `/generate-performance-analysis` to create statistical analysis and reports
