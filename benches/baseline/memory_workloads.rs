use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::collections::HashMap;

/// Baseline collection pipeline with multiple collect() calls and unnecessary allocations
fn baseline_collection_pipeline() -> Vec<i32> {
    let data: Vec<i32> = (0..10_000_000).collect();  // First collection
    
    // Multiple intermediate collections (very inefficient)
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

/// Baseline string building with concatenation and no pre-allocation
fn baseline_string_building() -> String {
    let mut result = String::new();  // No capacity pre-allocation
    
    for i in 0..100_000 {
        // Inefficient string concatenation - reallocates on every iteration
        result = result + &format!("Item {}: {}\n", i, i * i);
    }
    
    result
}

/// Baseline vector operations with frequent reallocations
fn baseline_vector_operations() -> Vec<Vec<i32>> {
    let mut result = Vec::new();  // No capacity pre-allocation
    
    for i in 0..10_000 {
        let mut inner = Vec::new();  // No capacity pre-allocation
        for j in 0..100 {
            inner.push(i * j);  // Frequent reallocations as vector grows
        }
        result.push(inner);  // More reallocations for outer vector
    }
    
    result
}

/// Baseline hash map operations with frequent rehashing
fn baseline_hashmap_operations() -> HashMap<String, i32> {
    let mut map = HashMap::new();  // No capacity hint
    
    for i in 0..1_000_000 {
        // Create new string for each key (inefficient)
        let key = format!("key_{}", i);
        map.insert(key, i);  // Frequent rehashing as map grows
    }
    
    // Additional operations that cause more allocations
    let mut result = HashMap::new();
    for (key, value) in map {
        let new_key = key.clone() + "_processed";  // More string allocations
        result.insert(new_key, value * 2);
    }
    
    result
}

/// Baseline nested data structure with excessive cloning
fn baseline_nested_structures() -> Vec<HashMap<String, Vec<String>>> {
    let mut result = Vec::new();
    
    for i in 0..1_000 {
        let mut map = HashMap::new();
        
        for j in 0..100 {
            let key = format!("group_{}", j);
            let mut values = Vec::new();
            
            for k in 0..50 {
                values.push(format!("item_{}_{}", i, k));
            }
            
            map.insert(key.clone(), values.clone());  // Unnecessary clones
        }
        
        result.push(map.clone());  // Clone entire HashMap
    }
    
    result
}

/// Baseline recursive data processing with stack allocations
fn baseline_recursive_processing() -> Vec<String> {
    let data: Vec<i32> = (0..100_000).collect();
    let mut result = Vec::new();
    
    for &item in &data {
        result.push(baseline_process_item(item, 0));
    }
    
    result
}

fn baseline_process_item(value: i32, depth: i32) -> String {
    if depth > 5 {
        return value.to_string();
    }
    
    // Recursive processing with string allocations at each level
    let processed = baseline_process_item(value * 2, depth + 1);
    format!("{}:{}", depth, processed)  // New string allocation
}

/// Baseline large object creation and manipulation
fn baseline_large_objects() -> Vec<LargeObject> {
    let mut objects = Vec::new();
    
    for i in 0..10_000 {
        let obj = LargeObject::new(i);
        objects.push(obj.clone());  // Clone large object
    }
    
    // Transform objects with more cloning
    let mut transformed = Vec::new();
    for obj in objects {
        let mut new_obj = obj.clone();
        new_obj.transform();
        transformed.push(new_obj);
    }
    
    transformed
}

#[derive(Clone)]
struct LargeObject {
    id: i32,
    data: Vec<String>,
    metadata: HashMap<String, String>,
    matrix: Vec<Vec<f64>>,
}

impl LargeObject {
    fn new(id: i32) -> Self {
        let mut data = Vec::new();
        for i in 0..100 {
            data.push(format!("data_item_{}", i));
        }
        
        let mut metadata = HashMap::new();
        for i in 0..50 {
            metadata.insert(format!("key_{}", i), format!("value_{}", i));
        }
        
        let mut matrix = Vec::new();
        for i in 0..20 {
            let mut row = Vec::new();
            for j in 0..20 {
                row.push((i * j) as f64);
            }
            matrix.push(row);
        }
        
        LargeObject {
            id,
            data,
            metadata,
            matrix,
        }
    }
    
    fn transform(&mut self) {
        // Transform with more allocations
        self.data = self.data.iter().map(|s| s.clone() + "_transformed").collect();
        
        for (key, value) in &self.metadata.clone() {
            self.metadata.insert(key.clone() + "_new", value.clone() + "_updated");
        }
    }
}

/// Baseline streaming data simulation with buffering issues
fn baseline_streaming_simulation() -> Vec<f64> {
    let mut result = Vec::new();
    
    // Simulate processing streaming data in small chunks (inefficient)
    for chunk_id in 0..1000 {
        let mut chunk_data = Vec::new();
        
        // Generate chunk data
        for i in 0..1000 {
            chunk_data.push((chunk_id * 1000 + i) as f64);
        }
        
        // Process chunk (with unnecessary intermediate collections)
        let processed: Vec<f64> = chunk_data
            .iter()
            .map(|&x| x * 2.0)
            .collect();
        
        let filtered: Vec<f64> = processed
            .iter()
            .filter(|&&x| x > 500.0)
            .cloned()
            .collect();
        
        // Append to result (causes frequent reallocations)
        for value in filtered {
            result.push(value);
        }
    }
    
    result
}

/// Baseline text processing with excessive string operations
fn baseline_text_processing() -> HashMap<String, usize> {
    let mut word_counts = HashMap::new();
    
    for i in 0..100_000 {
        // Generate text with string concatenation
        let mut text = String::new();
        for j in 0..10 {
            text = text + &format!("word{} ", (i + j) % 1000);
        }
        
        // Process text with more string allocations
        let words: Vec<String> = text
            .split_whitespace()
            .map(|w| w.to_lowercase())
            .collect();
        
        for word in words {
            let cleaned = word.replace("0", "zero")
                             .replace("1", "one")
                             .replace("2", "two");  // Multiple string allocations
            
            *word_counts.entry(cleaned.clone()).or_insert(0) += 1;
        }
    }
    
    word_counts
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
    
    c.bench_function("baseline_hashmap_operations", |b| {
        b.iter(|| black_box(baseline_hashmap_operations()))
    });
    
    c.bench_function("baseline_nested_structures", |b| {
        b.iter(|| black_box(baseline_nested_structures()))
    });
    
    c.bench_function("baseline_recursive_processing", |b| {
        b.iter(|| black_box(baseline_recursive_processing()))
    });
    
    c.bench_function("baseline_large_objects", |b| {
        b.iter(|| black_box(baseline_large_objects()))
    });
    
    c.bench_function("baseline_streaming_simulation", |b| {
        b.iter(|| black_box(baseline_streaming_simulation()))
    });
    
    c.bench_function("baseline_text_processing", |b| {
        b.iter(|| black_box(baseline_text_processing()))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
