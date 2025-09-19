use std::fs::File;
use std::io::{self, BufRead, BufReader};
use ahash::AHashMap;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use serde_json;
use rayon::prelude::*;

/// Optimized text tokenization with zero-copy and efficient hashing
fn optimized_text_tokenization() -> io::Result<AHashMap<String, usize>> {
    let file = File::open("/tmp/benchmark_data/text_corpus.txt")?;
    let reader = BufReader::with_capacity(64 * 1024, file);
    
    let mut word_counts = AHashMap::with_capacity(100_000); // Pre-allocate with estimated size
    
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

/// Optimized JSON processing with streaming and zero-copy where possible
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

/// Optimized JSON filtering with single-pass streaming
fn optimized_json_filtering() -> io::Result<Vec<serde_json::Value>> {
    let file = File::open("/tmp/benchmark_data/json_records.jsonl")?;
    let reader = BufReader::with_capacity(64 * 1024, file);
    
    let mut filtered_records = Vec::with_capacity(1_000_000); // Pre-allocate
    
    // Single pass: parse, filter, and transform in one go
    for line in reader.lines() {
        let line = line?;
        
        if let Ok(mut value) = serde_json::from_str::<serde_json::Value>(&line) {
            // Filter and transform in single pass
            if let Some(event_type) = value.get("event_type") {
                if event_type == "purchase" {
                    // Transform in place
                    if let Some(obj) = value.as_object_mut() {
                        obj.insert("processed".to_string(), serde_json::Value::Bool(true));
                    }
                    filtered_records.push(value);
                }
            }
        }
    }
    
    Ok(filtered_records)
}

/// Optimized text parsing with efficient string operations and regex-like functionality
fn optimized_text_parsing() -> io::Result<Vec<(String, String, String)>> {
    let file = File::open("/tmp/benchmark_data/text_corpus.txt")?;
    let reader = BufReader::with_capacity(64 * 1024, file);
    
    let mut extracted_data = Vec::with_capacity(10_000);
    
    for line in reader.lines() {
        let line = line?;
        
        // Efficient email extraction using byte-level processing
        for word in line.split_whitespace() {
            if let Some(at_pos) = word.find('@') {
                if let Some(dot_pos) = word[at_pos..].find('.') {
                    let username = &word[..at_pos];
                    let domain_start = at_pos + 1;
                    let domain_end = at_pos + dot_pos;
                    let domain = &word[domain_start..domain_end];
                    let tld = &word[domain_end + 1..];
                    
                    if !username.is_empty() && !domain.is_empty() && !tld.is_empty() {
                        extracted_data.push((
                            username.to_string(),
                            format!("{}.{}", domain, tld),
                            tld.to_string()
                        ));
                    }
                }
            }
        }
    }
    
    Ok(extracted_data)
}

/// Optimized CSV parsing with efficient field processing
fn optimized_csv_parsing() -> io::Result<Vec<Vec<String>>> {
    let file = File::open("/tmp/benchmark_data/large_data.csv")?;
    let reader = BufReader::with_capacity(64 * 1024, file);
    
    let mut records = Vec::with_capacity(1_000_000); // Pre-allocate
    
    for line in reader.lines() {
        let line = line?;
        
        // Efficient CSV parsing with pre-allocated capacity
        let field_count = line.matches(',').count() + 1;
        let mut fields = Vec::with_capacity(field_count);
        
        for field in line.split(',') {
            fields.push(field.trim().to_string());
        }
        
        records.push(fields);
    }
    
    Ok(records)
}

/// Optimized word frequency analysis with efficient data structures and algorithms
fn optimized_word_frequency() -> io::Result<Vec<(String, usize)>> {
    let file = File::open("/tmp/benchmark_data/text_corpus.txt")?;
    let reader = BufReader::with_capacity(64 * 1024, file);
    
    let mut word_counts = AHashMap::with_capacity(100_000);
    
    // Process text with minimal allocations
    for line in reader.lines() {
        let line = line?;
        
        // Process words directly without multiple string operations
        for word in line.split_whitespace() {
            // Clean word efficiently
            let cleaned: String = word
                .chars()
                .filter(|c| c.is_alphabetic())
                .map(|c| c.to_lowercase().next().unwrap())
                .collect();
            
            if !cleaned.is_empty() {
                *word_counts.entry(cleaned).or_insert(0) += 1;
            }
        }
    }
    
    // Convert to sorted vector efficiently
    let mut result: Vec<(String, usize)> = word_counts.into_iter().collect();
    result.sort_unstable_by(|a, b| b.1.cmp(&a.1)); // Unstable sort is faster
    
    Ok(result)
}

/// Optimized parallel text processing using Rayon
fn optimized_parallel_text_processing() -> io::Result<AHashMap<String, usize>> {
    let file = File::open("/tmp/benchmark_data/text_corpus.txt")?;
    let reader = BufReader::with_capacity(64 * 1024, file);
    
    // Collect lines for parallel processing
    let lines: Vec<String> = reader.lines().collect::<Result<Vec<_>, _>>()?;
    
    // Process lines in parallel and merge results
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
}

/// Optimized streaming parser with minimal memory usage
fn optimized_streaming_parser() -> io::Result<(usize, usize, usize)> {
    let file = File::open("/tmp/benchmark_data/json_records.jsonl")?;
    let reader = BufReader::with_capacity(64 * 1024, file);
    
    let mut json_count = 0;
    let mut purchase_count = 0;
    let mut total_value = 0.0;
    
    // Stream processing without storing all records
    for line in reader.lines() {
        let line = line?;
        
        if let Ok(value) = serde_json::from_str::<serde_json::Value>(&line) {
            json_count += 1;
            
            if let Some(event_type) = value.get("event_type") {
                if event_type == "purchase" {
                    purchase_count += 1;
                    
                    if let Some(val) = value.get("value").and_then(|v| v.as_f64()) {
                        total_value += val;
                    }
                }
            }
        }
    }
    
    Ok((json_count, purchase_count, total_value as usize))
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("optimized_text_tokenization", |b| {
        b.iter(|| black_box(optimized_text_tokenization().unwrap()))
    });
    
    c.bench_function("optimized_json_processing", |b| {
        b.iter(|| black_box(optimized_json_processing().unwrap()))
    });
    
    c.bench_function("optimized_json_filtering", |b| {
        b.iter(|| black_box(optimized_json_filtering().unwrap()))
    });
    
    c.bench_function("optimized_text_parsing", |b| {
        b.iter(|| black_box(optimized_text_parsing().unwrap()))
    });
    
    c.bench_function("optimized_csv_parsing", |b| {
        b.iter(|| black_box(optimized_csv_parsing().unwrap()))
    });
    
    c.bench_function("optimized_word_frequency", |b| {
        b.iter(|| black_box(optimized_word_frequency().unwrap()))
    });
    
    c.bench_function("optimized_parallel_text_processing", |b| {
        b.iter(|| black_box(optimized_parallel_text_processing().unwrap()))
    });
    
    c.bench_function("optimized_streaming_parser", |b| {
        b.iter(|| black_box(optimized_streaming_parser().unwrap()))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
