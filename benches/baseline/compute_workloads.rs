use std::fs::File;
use std::io::{self, Read};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

/// Baseline (naive) Jaro-Winkler implementation with character-by-character comparison
/// This represents a typical algorithmic implementation without optimization
fn baseline_jaro_winkler(s1: &str, s2: &str) -> f64 {
    // Convert to character vectors (inefficient)
    let s1_chars: Vec<char> = s1.chars().collect();
    let s2_chars: Vec<char> = s2.chars().collect();
    
    let len1 = s1_chars.len();
    let len2 = s2_chars.len();
    
    if len1 == 0 && len2 == 0 {
        return 1.0;
    }
    if len1 == 0 || len2 == 0 {
        return 0.0;
    }
    
    let match_window = std::cmp::max(len1, len2) / 2 - 1;
    let mut s1_matches = vec![false; len1];  // Allocate boolean vectors
    let mut s2_matches = vec![false; len2];
    
    let mut matches = 0;
    
    // Naive nested loop approach with bounds checking
    for i in 0..len1 {
        let start = if i >= match_window { i - match_window } else { 0 };
        let end = std::cmp::min(i + match_window + 1, len2);
        
        for j in start..end {
            if s2_matches[j] || s1_chars[i] != s2_chars[j] {
                continue;
            }
            s1_matches[i] = true;
            s2_matches[j] = true;
            matches += 1;
            break;
        }
    }
    
    if matches == 0 {
        return 0.0;
    }
    
    // Calculate transpositions with more inefficient loops
    let mut transpositions = 0;
    let mut k = 0;
    for i in 0..len1 {
        if !s1_matches[i] {
            continue;
        }
        while !s2_matches[k] {
            k += 1;
        }
        if s1_chars[i] != s2_chars[k] {
            transpositions += 1;
        }
        k += 1;
    }
    
    let jaro = (matches as f64 / len1 as f64 + 
                matches as f64 / len2 as f64 + 
                (matches as f64 - transpositions as f64 / 2.0) / matches as f64) / 3.0;
    
    // Simple prefix calculation with character-by-character comparison
    let mut prefix = 0;
    for i in 0..std::cmp::min(4, std::cmp::min(len1, len2)) {
        if s1_chars[i] == s2_chars[i] {
            prefix += 1;
        } else {
            break;
        }
    }
    
    jaro + 0.1 * prefix as f64 * (1.0 - jaro)
}

/// Baseline string similarity processing with inefficient file handling
fn baseline_string_similarity() -> io::Result<Vec<f64>> {
    let mut file = File::open("/tmp/benchmark_data/string_pairs.txt")?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    
    let mut similarities = Vec::new();  // No capacity pre-allocation
    
    for line in content.lines() {
        let parts: Vec<&str> = line.split('\t').collect();  // Collect into Vec
        if parts.len() >= 2 {
            let s1 = parts[0].to_string();  // Convert to owned strings
            let s2 = parts[1].to_string();
            let similarity = baseline_jaro_winkler(&s1, &s2);
            similarities.push(similarity);
        }
    }
    
    Ok(similarities)
}

/// Baseline numeric aggregation with bounds checking and multiple passes
fn baseline_numeric_aggregation() -> io::Result<(f64, f64, f64)> {
    let data = std::fs::read("/tmp/benchmark_data/numeric_data.bin")?;
    
    // Convert bytes to f64 values inefficiently with bounds checking
    let mut values = Vec::new();
    for i in (0..data.len()).step_by(8) {
        if i + 8 <= data.len() {  // Bounds check on every iteration
            let bytes = [
                data[i], data[i+1], data[i+2], data[i+3],
                data[i+4], data[i+5], data[i+6], data[i+7]
            ];
            values.push(f64::from_le_bytes(bytes));
        }
    }
    
    // Naive sequential processing with multiple passes (inefficient)
    let mut sum = 0.0;
    for i in 0..values.len() {  // Index-based loop with bounds checking
        sum += values[i];
    }
    let mean = sum / values.len() as f64;
    
    // Second pass for variance
    let mut variance_sum = 0.0;
    for i in 0..values.len() {
        let diff = values[i] - mean;
        variance_sum += diff * diff;
    }
    let std_dev = (variance_sum / values.len() as f64).sqrt();
    
    // Inefficient percentile calculation with full sort
    let mut sorted_values = values.clone();  // Clone the entire vector
    sorted_values.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let p95_index = (0.95 * sorted_values.len() as f64) as usize;
    let p95 = sorted_values[p95_index];
    
    Ok((mean, std_dev, p95))
}

/// Baseline prime number calculation with naive trial division
fn baseline_prime_calculation() -> Vec<usize> {
    let limit = 100_000;
    let mut primes = Vec::new();
    
    for n in 2..=limit {
        let mut is_prime = true;
        
        // Naive trial division - check all numbers up to n-1
        for i in 2..n {
            if n % i == 0 {
                is_prime = false;
                break;
            }
        }
        
        if is_prime {
            primes.push(n);
        }
    }
    
    primes
}

/// Baseline matrix operations with cache-unfriendly access patterns
fn baseline_matrix_operations() -> Vec<Vec<f64>> {
    let size = 500;  // Smaller size for baseline to avoid timeout
    
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
    
    // Naive triple nested loop (cache-unfriendly order)
    let mut result = Vec::new();
    for i in 0..size {
        let mut row = Vec::new();
        for j in 0..size {
            let mut sum = 0.0;
            for k in 0..size {
                // Cache-unfriendly access pattern
                sum += matrix_a[i][k] * matrix_b[k][j];
            }
            row.push(sum);
        }
        result.push(row);
    }
    
    result
}

/// Baseline fibonacci calculation with naive recursion
fn baseline_fibonacci(n: u32) -> u64 {
    if n <= 1 {
        n as u64
    } else {
        // Exponential time complexity - extremely inefficient
        baseline_fibonacci(n - 1) + baseline_fibonacci(n - 2)
    }
}

/// Baseline sorting algorithm - bubble sort (worst case O(n²))
fn baseline_sorting() -> Vec<i32> {
    // Generate test data
    let mut data: Vec<i32> = (0..10_000).rev().collect();  // Worst case: reverse sorted
    
    // Bubble sort implementation
    let n = data.len();
    for i in 0..n {
        for j in 0..n - 1 - i {
            if data[j] > data[j + 1] {
                data.swap(j, j + 1);
            }
        }
    }
    
    data
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("baseline_string_similarity", |b| {
        b.iter(|| black_box(baseline_string_similarity().unwrap()))
    });
    
    c.bench_function("baseline_numeric_aggregation", |b| {
        b.iter(|| black_box(baseline_numeric_aggregation().unwrap()))
    });
    
    c.bench_function("baseline_prime_calculation", |b| {
        b.iter(|| black_box(baseline_prime_calculation()))
    });
    
    c.bench_function("baseline_matrix_operations", |b| {
        b.iter(|| black_box(baseline_matrix_operations()))
    });
    
    c.bench_function("baseline_fibonacci", |b| {
        b.iter(|| black_box(baseline_fibonacci(35)))  // Small n to avoid timeout
    });
    
    c.bench_function("baseline_sorting", |b| {
        b.iter(|| black_box(baseline_sorting()))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
