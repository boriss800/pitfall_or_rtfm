use std::fs::File;
use std::io::{self, BufRead, BufReader};
use ahash::AHashMap;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rayon::prelude::*;

/// Optimized parallel word count using Rayon for multi-threaded processing
fn optimized_word_count() -> io::Result<AHashMap<String, usize>> {
    let file_paths = [
        "/tmp/benchmark_data/large_text.txt",
        "/tmp/benchmark_data/text_corpus.txt",
    ];
    
    // Process files in parallel
    let results: Result<Vec<_>, _> = file_paths
        .par_iter()
        .map(|&path| -> io::Result<AHashMap<String, usize>> {
            let file = File::open(path)?;
            let reader = BufReader::with_capacity(64 * 1024, file);
            
            // Process each file's lines in parallel
            let lines: Vec<String> = reader.lines().collect::<Result<Vec<_>, _>>()?;
            
            let word_counts: AHashMap<String, usize> = lines
                .par_iter()
                .map(|line| {
                    let mut local_counts = AHashMap::new();
                    for word in line.split_whitespace().map(|w| w.to_lowercase()) {
                        *local_counts.entry(word).or_insert(0) += 1;
                    }
                    local_counts
                })
                .reduce(
                    || AHashMap::new(),
                    |mut acc, local_counts| {
                        for (word, count) in local_counts {
                            *acc.entry(word).or_insert(0) += count;
                        }
                        acc
                    }
                );
            
            Ok(word_counts)
        })
        .collect();
    
    // Merge results from all files
    let file_results = results?;
    let global_counts = file_results
        .into_par_iter()
        .reduce(
            || AHashMap::new(),
            |mut acc, file_counts| {
                for (word, count) in file_counts {
                    *acc.entry(word).or_insert(0) += count;
                }
                acc
            }
        );
    
    Ok(global_counts)
}

/// Optimized parallel matrix multiplication using Rayon and cache-friendly blocking
fn optimized_matrix_multiplication() -> Vec<Vec<f64>> {
    let size = 800;
    
    // Pre-allocate matrices
    let matrix_a: Vec<Vec<f64>> = (0..size)
        .map(|i| (0..size).map(|j| (i * j) as f64).collect())
        .collect();
    
    let matrix_b: Vec<Vec<f64>> = (0..size)
        .map(|i| (0..size).map(|j| (i + j) as f64).collect())
        .collect();
    
    // Parallel matrix multiplication with cache-friendly blocking
    let mut result = vec![vec![0.0; size]; size];
    const BLOCK_SIZE: usize = 64;
    
    // Process blocks in parallel
    result
        .par_chunks_mut(BLOCK_SIZE)
        .enumerate()
        .for_each(|(block_i, result_block)| {
            let i_start = block_i * BLOCK_SIZE;
            let i_end = std::cmp::min(i_start + BLOCK_SIZE, size);
            
            for ii in (0..size).step_by(BLOCK_SIZE) {
                for kk in (0..size).step_by(BLOCK_SIZE) {
                    let ii_end = std::cmp::min(ii + BLOCK_SIZE, size);
                    let kk_end = std::cmp::min(kk + BLOCK_SIZE, size);
                    
                    for (local_i, i) in (i_start..i_end).enumerate() {
                        for k in kk..kk_end {
                            let a_ik = matrix_a[i][k];
                            for j in ii..ii_end {
                                result_block[local_i][j] += a_ik * matrix_b[k][j];
                            }
                        }
                    }
                }
            }
        });
    
    result
}

/// Optimized parallel image processing using Rayon
fn optimized_image_processing() -> Vec<Vec<u8>> {
    let width = 2000;
    let height = 2000;
    
    // Create simulated image
    let image: Vec<Vec<u8>> = (0..height)
        .map(|y| (0..width).map(|x| ((x + y) % 256) as u8).collect())
        .collect();
    
    // Apply filter in parallel
    let filtered: Vec<Vec<u8>> = image
        .par_iter()
        .enumerate()
        .map(|(y, row)| {
            row.iter()
                .enumerate()
                .map(|(x, _)| {
                    // 3x3 blur filter
                    let mut sum = 0u32;
                    let mut count = 0u32;
                    
                    for dy in -1i32..=1 {
                        for dx in -1i32..=1 {
                            let ny = y as i32 + dy;
                            let nx = x as i32 + dx;
                            
                            if ny >= 0 && ny < height as i32 && nx >= 0 && nx < width as i32 {
                                sum += image[ny as usize][nx as usize] as u32;
                                count += 1;
                            }
                        }
                    }
                    
                    (sum / count) as u8
                })
                .collect()
        })
        .collect();
    
    filtered
}

/// Optimized parallel Monte Carlo simulation
fn optimized_monte_carlo_pi() -> f64 {
    let iterations = 10_000_000;
    let chunk_size = 100_000;
    
    // Process chunks in parallel
    let inside_circle: usize = (0..iterations)
        .into_par_iter()
        .chunks(chunk_size)
        .map(|chunk| {
            let mut local_inside = 0;
            
            for _i in chunk {
                // Use better random number generation
                let x: f64 = rand::random();
                let y: f64 = rand::random();
                
                if x * x + y * y <= 1.0 {
                    local_inside += 1;
                }
            }
            
            local_inside
        })
        .sum();
    
    4.0 * inside_circle as f64 / iterations as f64
}

/// Optimized parallel sum using SIMD and Rayon
fn optimized_parallel_sum() -> i64 {
    let data: Vec<i32> = (0..10_000_000).collect();
    
    // Parallel sum with optimal chunk size
    let sum: i64 = data
        .par_chunks(10_000)
        .map(|chunk| chunk.iter().map(|&x| x as i64).sum::<i64>())
        .sum();
    
    sum
}

/// Optimized parallel search using Rayon
fn optimized_parallel_search() -> Vec<usize> {
    let datasets = vec![
        (0..1_000_000).collect::<Vec<i32>>(),
        (1_000_000..2_000_000).collect::<Vec<i32>>(),
        (2_000_000..3_000_000).collect::<Vec<i32>>(),
        (3_000_000..4_000_000).collect::<Vec<i32>>(),
    ];
    
    let search_values = vec![500_000, 1_500_000, 2_500_000, 3_500_000];
    
    // Search datasets in parallel
    let results: Vec<usize> = datasets
        .par_iter()
        .zip(search_values.par_iter())
        .map(|(dataset, &target)| {
            // Use binary search since data is sorted
            dataset.binary_search(&target).unwrap_or(0)
        })
        .collect();
    
    results
}

/// Optimized parallel recursive computation
fn optimized_recursive_computation() -> Vec<u64> {
    let inputs = vec![20, 21, 22, 23, 24, 25, 26, 27, 28, 29];
    
    // Compute factorials in parallel using iterative approach
    let results: Vec<u64> = inputs
        .par_iter()
        .map(|&n| optimized_factorial(n))
        .collect();
    
    results
}

fn optimized_factorial(n: u64) -> u64 {
    // Iterative factorial (much faster than recursive)
    (1..=n).product()
}

/// Optimized parallel data transformation pipeline
fn optimized_data_pipeline() -> Vec<f64> {
    let data: Vec<i32> = (0..5_000_000).collect();
    
    // Single parallel pass through the entire pipeline
    let result: Vec<f64> = data
        .par_iter()
        .filter_map(|&x| {
            if x % 2 == 0 {
                let doubled = x * 2;
                let scaled = doubled as f64 * 1.5;
                if scaled > 1000.0 {
                    Some(scaled.sqrt())
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect();
    
    result
}

/// Optimized parallel reduce operations
fn optimized_parallel_reduce() -> (i64, f64, i32, i32) {
    let data: Vec<i32> = (0..10_000_000).collect();
    
    // Multiple reductions in parallel
    let (sum, mean, min, max) = data
        .par_iter()
        .fold(
            || (0i64, 0.0, i32::MAX, i32::MIN),
            |acc, &x| {
                (
                    acc.0 + x as i64,
                    acc.1 + x as f64,
                    acc.2.min(x),
                    acc.3.max(x),
                )
            }
        )
        .reduce(
            || (0i64, 0.0, i32::MAX, i32::MIN),
            |acc1, acc2| {
                (
                    acc1.0 + acc2.0,
                    acc1.1 + acc2.1,
                    acc1.2.min(acc2.2),
                    acc1.3.max(acc2.3),
                )
            }
        );
    
    let mean = mean / data.len() as f64;
    (sum, mean, min, max)
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("optimized_word_count", |b| {
        b.iter(|| black_box(optimized_word_count().unwrap()))
    });
    
    c.bench_function("optimized_matrix_multiplication", |b| {
        b.iter(|| black_box(optimized_matrix_multiplication()))
    });
    
    c.bench_function("optimized_image_processing", |b| {
        b.iter(|| black_box(optimized_image_processing()))
    });
    
    c.bench_function("optimized_monte_carlo_pi", |b| {
        b.iter(|| black_box(optimized_monte_carlo_pi()))
    });
    
    c.bench_function("optimized_parallel_sum", |b| {
        b.iter(|| black_box(optimized_parallel_sum()))
    });
    
    c.bench_function("optimized_parallel_search", |b| {
        b.iter(|| black_box(optimized_parallel_search()))
    });
    
    c.bench_function("optimized_recursive_computation", |b| {
        b.iter(|| black_box(optimized_recursive_computation()))
    });
    
    c.bench_function("optimized_data_pipeline", |b| {
        b.iter(|| black_box(optimized_data_pipeline()))
    });
    
    c.bench_function("optimized_parallel_reduce", |b| {
        b.iter(|| black_box(optimized_parallel_reduce()))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
