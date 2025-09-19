# Implement Baseline Benchmarks

**Keywords:** baseline, naive, implementation, debug, unoptimized, benchmarks
**Flags:** `day-2`, `baseline-implementation`, `naive-code`

## Description
Implement all baseline (naive) benchmark implementations that represent typical developer code without optimizations. These serve as the control group for performance comparisons.

## Critical Requirements
- **NEVER use release mode** for baseline benchmarks
- **NEVER apply optimizations** - code should represent what developers actually write
- **Always use debug builds** with default settings
- **Include frequent allocations** and inefficient patterns
- **Use index-based loops** with bounds checking
- **Apply liberal cloning** instead of borrowing

## Steps

### 1. Implement I/O Baseline Benchmarks
Create `benches/baseline/io_workloads.rs`:

```rust
use std::fs::File;
use std::io::{self, Read, Write};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

// Naive file reading - no buffering, load entire file into memory
fn baseline_large_file_processing() -> io::Result<usize> {
    let mut file = File::open("/tmp/benchmark_data/large_text.txt")?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;  // Load entire 1GB file into memory
    
    let mut word_count = 0;
    for line in content.lines() {
        // Inefficient: creates new Vec for each line
        let words: Vec<&str> = line.split_whitespace().collect();
        word_count += words.len();
    }
    Ok(word_count)
}

// Naive CSV transformation with string concatenation
fn baseline_csv_transformation() -> io::Result<()> {
    let mut file = File::open("/tmp/benchmark_data/large_data.csv")?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    
    let mut output = String::new();  // No capacity pre-allocation
    for line in content.lines() {
        let fields: Vec<&str> = line.split(',').collect();  // Unnecessary collection
        for field in fields {
            // Inefficient string concatenation
            output = output + &field.to_uppercase() + ",";
        }
        output = output + "\n";  // More inefficient concatenation
    }
    
    let mut output_file = File::create("/tmp/benchmark_data/output_baseline.csv")?;
    output_file.write_all(output.as_bytes())?;
    Ok(())
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("baseline_large_file_processing", |b| {
        b.iter(|| black_box(baseline_large_file_processing().unwrap()))
    });
    
    c.bench_function("baseline_csv_transformation", |b| {
        b.iter(|| black_box(baseline_csv_transformation().unwrap()))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
```

### 2. Implement Parsing Baseline Benchmarks
Create `benches/baseline/parsing_workloads.rs`:

```rust
use std::fs::File;
use std::io::{self, Read};
use std::collections::HashMap;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use serde_json;

// Naive text tokenization with frequent allocations
fn baseline_text_tokenization() -> io::Result<HashMap<String, usize>> {
    let mut file = File::open("/tmp/benchmark_data/text_corpus.txt")?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    
    let mut word_counts = HashMap::new();
    
    for line in content.lines() {
        // Inefficient: collect into Vec first
        let words: Vec<String> = line
            .split_whitespace()
            .map(|w| w.to_lowercase().clone())  // Unnecessary clone
            .collect();
        
        for word in words {
            // Clone the word for HashMap key
            let count = word_counts.entry(word.clone()).or_insert(0);
            *count += 1;
        }
    }
    
    Ok(word_counts)
}

// Naive JSON parsing with string allocations
fn baseline_json_processing() -> io::Result<Vec<serde_json::Value>> {
    let mut file = File::open("/tmp/benchmark_data/json_records.jsonl")?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    
    let mut records = Vec::new();  // No capacity pre-allocation
    
    for line in content.lines() {
        // Parse each line as separate JSON, creating new strings
        let json_str = line.to_string();  // Unnecessary string allocation
        match serde_json::from_str::<serde_json::Value>(&json_str) {
            Ok(value) => {
                records.push(value.clone());  // Clone the entire JSON value
            }
            Err(_) => continue,
        }
    }
    
    Ok(records)
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("baseline_text_tokenization", |b| {
        b.iter(|| black_box(baseline_text_tokenization().unwrap()))
    });
    
    c.bench_function("baseline_json_processing", |b| {
        b.iter(|| black_box(baseline_json_processing().unwrap()))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
```

### 3. Implement Computational Baseline Benchmarks
Create `benches/baseline/compute_workloads.rs`:

```rust
use std::fs::File;
use std::io::{self, Read};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

// Naive Jaro-Winkler implementation with character-by-character comparison
fn baseline_jaro_winkler(s1: &str, s2: &str) -> f64 {
    let s1_chars: Vec<char> = s1.chars().collect();  // Collect into Vec
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
    
    // Naive nested loop approach
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
    
    // Simple prefix calculation
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

// Naive string similarity processing
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

// Naive numeric aggregation with bounds checking
fn baseline_numeric_aggregation() -> io::Result<(f64, f64, f64)> {
    let data = std::fs::read("/tmp/benchmark_data/numeric_data.bin")?;
    
    // Convert bytes to f64 values inefficiently
    let mut values = Vec::new();
    for i in (0..data.len()).step_by(8) {
        if i + 8 <= data.len() {
            let bytes = [
                data[i], data[i+1], data[i+2], data[i+3],
                data[i+4], data[i+5], data[i+6], data[i+7]
            ];
            values.push(f64::from_le_bytes(bytes));
        }
    }
    
    // Naive sequential processing with multiple passes
    let mut sum = 0.0;
    for i in 0..values.len() {  // Index-based loop with bounds checking
        sum += values[i];
    }
    let mean = sum / values.len() as f64;
    
    let mut variance_sum = 0.0;
    for i in 0..values.len() {
        let diff = values[i] - mean;
        variance_sum += diff * diff;
    }
    let std_dev = (variance_sum / values.len() as f64).sqrt();
    
    // Inefficient percentile calculation
    let mut sorted_values = values.clone();  // Clone the entire vector
    sorted_values.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let p95_index = (0.95 * sorted_values.len() as f64) as usize;
    let p95 = sorted_values[p95_index];
    
    Ok((mean, std_dev, p95))
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("baseline_string_similarity", |b| {
        b.iter(|| black_box(baseline_string_similarity().unwrap()))
    });
    
    c.bench_function("baseline_numeric_aggregation", |b| {
        b.iter(|| black_box(baseline_numeric_aggregation().unwrap()))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
```

### 4. Implement Parallel Baseline Benchmarks
Create `benches/baseline/parallel_workloads.rs`:

```rust
use std::fs::File;
use std::io::{self, Read};
use std::collections::HashMap;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

// Naive sequential word count (no parallelism)
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

// Naive matrix multiplication (single-threaded)
fn baseline_matrix_multiplication() -> Vec<Vec<f64>> {
    let size = 1000;
    
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
    
    // Naive triple nested loop (cache-unfriendly)
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

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("baseline_word_count", |b| {
        b.iter(|| black_box(baseline_word_count().unwrap()))
    });
    
    c.bench_function("baseline_matrix_multiplication", |b| {
        b.iter(|| black_box(baseline_matrix_multiplication()))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
```

### 5. Implement Memory Baseline Benchmarks
Create `benches/baseline/memory_workloads.rs`:

```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};

// Naive collection pipeline with multiple collect() calls
fn baseline_collection_pipeline() -> Vec<i32> {
    let data: Vec<i32> = (0..10_000_000).collect();  // First collection
    
    // Multiple intermediate collections
    let filtered: Vec<i32> = data
        .iter()
        .filter(|&&x| x % 2 == 0)
        .cloned()  // Unnecessary clone
        .collect();  // Second collection
    
    let mapped: Vec<i32> = filtered
        .iter()
        .map(|&x| x * 2)
        .collect();  // Third collection
    
    let final_result: Vec<i32> = mapped
        .iter()
        .filter(|&&x| x > 1000)
        .cloned()  // Another unnecessary clone
        .collect();  // Fourth collection
    
    final_result
}

// Naive string building with concatenation
fn baseline_string_building() -> String {
    let mut result = String::new();  // No capacity pre-allocation
    
    for i in 0..100_000 {
        // Inefficient string concatenation
        result = result + &format!("Item {}: {}\n", i, i * i);
    }
    
    result
}

// Naive vector operations with frequent reallocations
fn baseline_vector_operations() -> Vec<Vec<i32>> {
    let mut result = Vec::new();  // No capacity pre-allocation
    
    for i in 0..10_000 {
        let mut inner = Vec::new();  // No capacity pre-allocation
        for j in 0..100 {
            inner.push(i * j);  // Frequent reallocations
        }
        result.push(inner);  // More reallocations
    }
    
    result
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("baseline_collection_pipeline", |b| {
        b.iter(|| black_box(baseline_collection_pipeline()))
    });
    
    c.bench_function("baseline_string_building", |b| {
        b.iter(|| black_box(baseline_string_building()))
    });
    
    c.bench_function("baseline_vector_operations", |b| {
        b.iter(|| black_box(baseline_vector_operations()))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
```

## Validation Checklist
- [ ] All benchmarks use debug builds (no optimizations)
- [ ] Code represents realistic naive implementations
- [ ] Frequent allocations and cloning are included
- [ ] Index-based loops are used instead of iterators
- [ ] No SIMD, parallelism, or advanced optimizations
- [ ] String concatenation uses `+` operator
- [ ] Collections are created without capacity pre-allocation
- [ ] All benchmarks use `criterion::black_box`
- [ ] File paths point to tmpfs locations
- [ ] Error handling is basic but functional

## Next Steps
After completing baseline implementations:
1. Run `/implement-optimized-benchmarks` to create optimized versions
2. Run `/validate-benchmark-correctness` to ensure both versions produce same results
3. Run `/execute-initial-benchmarks` to get first performance measurements
