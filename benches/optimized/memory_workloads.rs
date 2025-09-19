use criterion::{black_box, criterion_group, criterion_main, Criterion};
use ahash::AHashMap;
use rayon::prelude::*;

/// Optimized collection pipeline using streaming iterators and no intermediate collections
fn optimized_collection_pipeline() -> Vec<i32> {
    let data: Vec<i32> = (0..10_000_000).collect();
    
    // Single streaming pass - no intermediate collections
    data.into_iter()
        .filter(|&x| x % 2 == 0)
        .map(|x| x * 2)
        .filter(|&x| x > 1000)
        .collect()
}

/// Optimized string building with pre-allocation and efficient concatenation
fn optimized_string_building() -> String {
    // Pre-calculate required capacity
    let count = 100_000;
    let estimated_size = count * 20; // Rough estimate per line
    let mut result = String::with_capacity(estimated_size);
    
    for i in 0..count {
        // Use write! macro for efficient formatting
        use std::fmt::Write;
        write!(result, "Item {}: {}\n", i, i * i).unwrap();
    }
    
    result
}

/// Optimized vector operations with pre-allocation
fn optimized_vector_operations() -> Vec<Vec<i32>> {
    let outer_size = 10_000;
    let inner_size = 100;
    
    // Pre-allocate outer vector with exact capacity
    let mut result = Vec::with_capacity(outer_size);
    
    for i in 0..outer_size {
        // Pre-allocate inner vector with exact capacity
        let mut inner = Vec::with_capacity(inner_size);
        for j in 0..inner_size {
            inner.push((i * j) as i32);
        }
        result.push(inner);
    }
    
    result
}

/// Optimized hash map operations with capacity hints and efficient hashing
fn optimized_hashmap_operations() -> AHashMap<String, i32> {
    let count = 1_000_000;
    
    // Pre-allocate with exact capacity to avoid rehashing
    let mut map = AHashMap::with_capacity(count);
    
    for i in 0..count {
        // Use format! only once, reuse the key
        let key = format!("key_{}", i);
        map.insert(key, i as i32);
    }
    
    // Transform in-place to avoid additional allocations
    for (_, value) in &mut map {
        *value *= 2;
    }
    
    map
}

/// Optimized nested structures with minimal cloning and smart allocation
fn optimized_nested_structures() -> Vec<AHashMap<String, Vec<String>>> {
    let outer_count = 1_000;
    let inner_count = 100;
    let values_count = 50;
    
    // Pre-allocate result vector
    let mut result = Vec::with_capacity(outer_count);
    
    for i in 0..outer_count {
        // Pre-allocate map with capacity
        let mut map = AHashMap::with_capacity(inner_count);
        
        for j in 0..inner_count {
            let key = format!("group_{}", j);
            
            // Pre-allocate values vector
            let mut values = Vec::with_capacity(values_count);
            for k in 0..values_count {
                values.push(format!("item_{}_{}", i, k));
            }
            
            map.insert(key, values);
        }
        
        result.push(map);
    }
    
    result
}

/// Optimized recursive processing using iterative approach and pre-allocation
fn optimized_recursive_processing() -> Vec<String> {
    let data: Vec<i32> = (0..100_000).collect();
    let mut result = Vec::with_capacity(data.len());
    
    for &item in &data {
        // Use iterative approach instead of recursion
        result.push(optimized_process_item(item));
    }
    
    result
}

fn optimized_process_item(mut value: i32) -> String {
    // Iterative processing instead of recursive
    let mut parts = Vec::with_capacity(6); // Pre-allocate for depth
    
    for depth in 0..=5 {
        parts.push(format!("{}:{}", depth, value));
        value *= 2;
    }
    
    parts.join(":")
}

/// Optimized large object creation with efficient memory management
fn optimized_large_objects() -> Vec<OptimizedLargeObject> {
    let count = 10_000;
    let mut objects = Vec::with_capacity(count);
    
    for i in 0..count {
        objects.push(OptimizedLargeObject::new(i as i32));
    }
    
    // Transform in-place to avoid cloning
    for obj in &mut objects {
        obj.transform_in_place();
    }
    
    objects
}

struct OptimizedLargeObject {
    id: i32,
    data: Vec<String>,
    metadata: AHashMap<String, String>,
    matrix: Vec<Vec<f64>>,
}

impl OptimizedLargeObject {
    fn new(id: i32) -> Self {
        // Pre-allocate all collections with known sizes
        let mut data = Vec::with_capacity(100);
        for i in 0..100 {
            data.push(format!("data_item_{}", i));
        }
        
        let mut metadata = AHashMap::with_capacity(50);
        for i in 0..50 {
            metadata.insert(format!("key_{}", i), format!("value_{}", i));
        }
        
        let mut matrix = Vec::with_capacity(20);
        for i in 0..20 {
            let mut row = Vec::with_capacity(20);
            for j in 0..20 {
                row.push((i * j) as f64);
            }
            matrix.push(row);
        }
        
        OptimizedLargeObject {
            id,
            data,
            metadata,
            matrix,
        }
    }
    
    fn transform_in_place(&mut self) {
        // Transform in-place to avoid allocations
        for item in &mut self.data {
            item.push_str("_transformed");
        }
        
        // Extend existing map instead of creating new entries
        let keys_to_update: Vec<String> = self.metadata.keys().cloned().collect();
        for key in keys_to_update {
            if let Some(value) = self.metadata.get_mut(&key) {
                value.push_str("_updated");
            }
        }
    }
}

/// Optimized streaming simulation with efficient buffering
fn optimized_streaming_simulation() -> Vec<f64> {
    let chunk_count = 1000;
    let chunk_size = 1000;
    let total_size = chunk_count * chunk_size;
    
    // Pre-allocate result with exact capacity
    let mut result = Vec::with_capacity(total_size / 2); // Estimate after filtering
    
    for chunk_id in 0..chunk_count {
        // Process chunk in streaming fashion
        let chunk_start = chunk_id * chunk_size;
        
        for i in 0..chunk_size {
            let value = (chunk_start + i) as f64 * 2.0;
            if value > 500.0 {
                result.push(value);
            }
        }
    }
    
    result.shrink_to_fit(); // Optimize final memory usage
    result
}

/// Optimized text processing with efficient string operations and pre-allocation
fn optimized_text_processing() -> AHashMap<String, usize> {
    let count = 100_000;
    let mut word_counts = AHashMap::with_capacity(10_000); // Estimate unique words
    
    for i in 0..count {
        // Generate words efficiently
        for j in 0..10 {
            let word_id = (i + j) % 1000;
            let word = format!("word{}", word_id);
            
            // Process word efficiently - avoid multiple string operations
            let cleaned = if word.contains('0') {
                word.replace('0', "zero")
            } else if word.contains('1') {
                word.replace('1', "one")
            } else if word.contains('2') {
                word.replace('2', "two")
            } else {
                word
            };
            
            *word_counts.entry(cleaned).or_insert(0) += 1;
        }
    }
    
    word_counts
}

/// Optimized parallel memory operations using Rayon
fn optimized_parallel_memory_operations() -> Vec<Vec<i32>> {
    let size = 10_000;
    
    // Parallel generation with pre-allocation
    let result: Vec<Vec<i32>> = (0..size)
        .into_par_iter()
        .map(|i| {
            let mut row = Vec::with_capacity(100);
            for j in 0..100 {
                row.push(i * j);
            }
            row
        })
        .collect();
    
    result
}

/// Optimized zero-copy string processing
fn optimized_zero_copy_processing() -> Vec<usize> {
    let text = "performance optimization rust benchmark memory allocation simd parallel rayon iterator zero copy buffer cache throughput latency efficiency algorithm data structure vector string processing computation analysis measurement".repeat(10_000);
    
    // Zero-copy string processing using string slices
    let word_lengths: Vec<usize> = text
        .split_whitespace()
        .map(|word| word.len()) // No allocation, just length calculation
        .collect();
    
    word_lengths
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
    
    c.bench_function("optimized_hashmap_operations", |b| {
        b.iter(|| black_box(optimized_hashmap_operations()))
    });
    
    c.bench_function("optimized_nested_structures", |b| {
        b.iter(|| black_box(optimized_nested_structures()))
    });
    
    c.bench_function("optimized_recursive_processing", |b| {
        b.iter(|| black_box(optimized_recursive_processing()))
    });
    
    c.bench_function("optimized_large_objects", |b| {
        b.iter(|| black_box(optimized_large_objects()))
    });
    
    c.bench_function("optimized_streaming_simulation", |b| {
        b.iter(|| black_box(optimized_streaming_simulation()))
    });
    
    c.bench_function("optimized_text_processing", |b| {
        b.iter(|| black_box(optimized_text_processing()))
    });
    
    c.bench_function("optimized_parallel_memory_operations", |b| {
        b.iter(|| black_box(optimized_parallel_memory_operations()))
    });
    
    c.bench_function("optimized_zero_copy_processing", |b| {
        b.iter(|| black_box(optimized_zero_copy_processing()))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
