use std::fs::File;
use std::io::{self, Read};
use std::collections::HashMap;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

/// Baseline (sequential) word count - no parallelism, processes files one by one
fn baseline_word_count() -> io::Result<HashMap<String, usize>> {
    let file_paths = [
        "/tmp/benchmark_data/large_text.txt",
        "/tmp/benchmark_data/text_corpus.txt",
    ];
    
    let mut global_counts = HashMap::new();
    
    // Process files sequentially, one at a time
    for path in &file_paths {
        let mut file = File::open(path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        
        // Sequential processing of each file
        for line in content.lines() {
            let words: Vec<String> = line
                .split_whitespace()
                .map(|w| w.to_lowercase())
                .collect();  // Collect into Vec first
            
            for word in words {
                let count = global_counts.entry(word.clone()).or_insert(0);
                *count += 1;
            }
        }
    }
    
    Ok(global_counts)
}

/// Baseline matrix multiplication (single-threaded, cache-unfriendly)
fn baseline_matrix_multiplication() -> Vec<Vec<f64>> {
    let size = 800;  // Reasonable size for baseline
    
    // Create matrices with inefficient initialization
    let mut matrix_a = Vec::new();
    let mut matrix_b = Vec::new();
    
    for i in 0..size {
        let mut row_a = Vec::new();
        let mut row_b = Vec::new();
        for j in 0..size {
            row_a.push((i * j) as f64);
            row_b.push((i + j) as f64);
        }
        matrix_a.push(row_a);
        matrix_b.push(row_b);
    }
    
    // Naive triple nested loop (cache-unfriendly access pattern)
    let mut result = Vec::new();
    for i in 0..size {
        let mut row = Vec::new();
        for j in 0..size {
            let mut sum = 0.0;
            for k in 0..size {
                // Cache-unfriendly access pattern - accessing matrix_b column-wise
                sum += matrix_a[i][k] * matrix_b[k][j];
            }
            row.push(sum);
        }
        result.push(row);
    }
    
    result
}

/// Baseline image processing simulation (sequential pixel processing)
fn baseline_image_processing() -> Vec<Vec<u8>> {
    let width = 2000;
    let height = 2000;
    
    // Create a simulated image
    let mut image = Vec::new();
    for y in 0..height {
        let mut row = Vec::new();
        for x in 0..width {
            row.push(((x + y) % 256) as u8);
        }
        image.push(row);
    }
    
    // Apply a simple filter sequentially
    let mut filtered = Vec::new();
    for y in 0..height {
        let mut row = Vec::new();
        for x in 0..width {
            // Simple blur filter (3x3 kernel)
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
            
            row.push((sum / count) as u8);
        }
        filtered.push(row);
    }
    
    filtered
}

/// Baseline Monte Carlo simulation (sequential)
fn baseline_monte_carlo_pi() -> f64 {
    let iterations = 10_000_000;
    let mut inside_circle = 0;
    
    // Sequential random sampling
    for i in 0..iterations {
        // Simple linear congruential generator (not thread-safe)
        let x = ((i * 1103515245 + 12345) % (1 << 31)) as f64 / (1 << 31) as f64;
        let y = ((i * 1664525 + 1013904223) % (1 << 31)) as f64 / (1 << 31) as f64;
        
        if x * x + y * y <= 1.0 {
            inside_circle += 1;
        }
    }
    
    4.0 * inside_circle as f64 / iterations as f64
}

/// Baseline parallel sum simulation (sequential processing)
fn baseline_parallel_sum() -> i64 {
    let data: Vec<i32> = (0..10_000_000).collect();
    
    // Sequential summation
    let mut total = 0i64;
    for &value in &data {
        total += value as i64;
    }
    
    total
}

/// Baseline search in multiple datasets (sequential)
fn baseline_parallel_search() -> Vec<usize> {
    let datasets = vec![
        (0..1_000_000).collect::<Vec<i32>>(),
        (1_000_000..2_000_000).collect::<Vec<i32>>(),
        (2_000_000..3_000_000).collect::<Vec<i32>>(),
        (3_000_000..4_000_000).collect::<Vec<i32>>(),
    ];
    
    let search_values = vec![500_000, 1_500_000, 2_500_000, 3_500_000];
    let mut results = Vec::new();
    
    // Search each dataset sequentially
    for (i, dataset) in datasets.iter().enumerate() {
        let target = search_values[i];
        
        // Linear search (inefficient)
        for (pos, &value) in dataset.iter().enumerate() {
            if value == target {
                results.push(pos);
                break;
            }
        }
    }
    
    results
}

/// Baseline recursive computation (sequential)
fn baseline_recursive_computation() -> Vec<u64> {
    let inputs = vec![20, 21, 22, 23, 24, 25, 26, 27, 28, 29];
    let mut results = Vec::new();
    
    // Compute factorial for each input sequentially
    for &n in &inputs {
        results.push(baseline_factorial(n));
    }
    
    results
}

fn baseline_factorial(n: u64) -> u64 {
    if n <= 1 {
        1
    } else {
        n * baseline_factorial(n - 1)
    }
}

/// Baseline data transformation pipeline (sequential)
fn baseline_data_pipeline() -> Vec<f64> {
    let data: Vec<i32> = (0..5_000_000).collect();
    
    // Sequential pipeline processing
    let mut step1 = Vec::new();
    for &x in &data {
        if x % 2 == 0 {
            step1.push(x);
        }
    }
    
    let mut step2 = Vec::new();
    for &x in &step1 {
        step2.push(x * 2);
    }
    
    let mut step3 = Vec::new();
    for &x in &step2 {
        step3.push(x as f64 * 1.5);
    }
    
    let mut result = Vec::new();
    for &x in &step3 {
        if x > 1000.0 {
            result.push(x.sqrt());
        }
    }
    
    result
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("baseline_word_count", |b| {
        b.iter(|| black_box(baseline_word_count().unwrap()))
    });
    
    c.bench_function("baseline_matrix_multiplication", |b| {
        b.iter(|| black_box(baseline_matrix_multiplication()))
    });
    
    c.bench_function("baseline_image_processing", |b| {
        b.iter(|| black_box(baseline_image_processing()))
    });
    
    c.bench_function("baseline_monte_carlo_pi", |b| {
        b.iter(|| black_box(baseline_monte_carlo_pi()))
    });
    
    c.bench_function("baseline_parallel_sum", |b| {
        b.iter(|| black_box(baseline_parallel_sum()))
    });
    
    c.bench_function("baseline_parallel_search", |b| {
        b.iter(|| black_box(baseline_parallel_search()))
    });
    
    c.bench_function("baseline_recursive_computation", |b| {
        b.iter(|| black_box(baseline_recursive_computation()))
    });
    
    c.bench_function("baseline_data_pipeline", |b| {
        b.iter(|| black_box(baseline_data_pipeline()))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
