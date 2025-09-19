use std::fs::File;
use std::io::{self, Read};
use std::collections::HashMap;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use serde_json;

/// Baseline text tokenization with frequent allocations and inefficient patterns
fn baseline_text_tokenization() -> io::Result<HashMap<String, usize>> {
    let mut file = File::open("/tmp/benchmark_data/text_corpus.txt")?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    
    let mut word_counts = HashMap::new();  // Use standard HashMap (slower than AHashMap)
    
    for line in content.lines() {
        // Inefficient: collect into Vec first, then process
        let words: Vec<String> = line
            .split_whitespace()
            .map(|w| w.to_lowercase().clone())  // Unnecessary clone
            .collect();
        
        for word in words {
            // Clone the word for HashMap key (inefficient)
            let count = word_counts.entry(word.clone()).or_insert(0);
            *count += 1;
        }
    }
    
    Ok(word_counts)
}

/// Baseline JSON parsing with string allocations and inefficient processing
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

/// Baseline JSON filtering with multiple passes and allocations
fn baseline_json_filtering() -> io::Result<Vec<serde_json::Value>> {
    let mut file = File::open("/tmp/benchmark_data/json_records.jsonl")?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    
    // First pass: parse all records
    let mut all_records = Vec::new();
    for line in content.lines() {
        let json_str = line.to_string();
        if let Ok(value) = serde_json::from_str::<serde_json::Value>(&json_str) {
            all_records.push(value.clone());
        }
    }
    
    // Second pass: filter records (inefficient multiple iterations)
    let mut filtered_records = Vec::new();
    for record in &all_records {
        if let Some(event_type) = record.get("event_type") {
            if event_type == "purchase" {
                filtered_records.push(record.clone());  // Clone again
            }
        }
    }
    
    // Third pass: transform records
    let mut final_records = Vec::new();
    for record in &filtered_records {
        let mut new_record = record.clone();  // Yet another clone
        if let Some(obj) = new_record.as_object_mut() {
            obj.insert("processed".to_string(), serde_json::Value::Bool(true));
        }
        final_records.push(new_record);
    }
    
    Ok(final_records)
}

/// Baseline text parsing with regex-like functionality using string methods
fn baseline_text_parsing() -> io::Result<Vec<(String, String, String)>> {
    let mut file = File::open("/tmp/benchmark_data/text_corpus.txt")?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    
    let mut extracted_data = Vec::new();
    
    for line in content.lines() {
        let line_str = line.to_string();  // Convert to owned string
        
        // Naive email extraction (without regex)
        let words: Vec<&str> = line_str.split_whitespace().collect();
        for word in words {
            if word.contains("@") && word.contains(".") {
                let parts: Vec<&str> = word.split("@").collect();
                if parts.len() == 2 {
                    let username = parts[0].to_string();
                    let domain_parts: Vec<&str> = parts[1].split(".").collect();
                    if domain_parts.len() >= 2 {
                        let domain = parts[1].to_string();
                        let tld = domain_parts.last().unwrap().to_string();
                        extracted_data.push((username, domain, tld));
                    }
                }
            }
        }
    }
    
    Ok(extracted_data)
}

/// Baseline CSV parsing without proper CSV library
fn baseline_csv_parsing() -> io::Result<Vec<Vec<String>>> {
    let mut file = File::open("/tmp/benchmark_data/large_data.csv")?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    
    let mut records = Vec::new();
    
    for line in content.lines() {
        // Naive CSV parsing - doesn't handle quoted fields properly
        let fields: Vec<String> = line
            .split(',')
            .map(|field| field.trim().to_string())  // Allocate string for each field
            .collect();
        
        records.push(fields);
    }
    
    Ok(records)
}

/// Baseline word frequency analysis with inefficient data structures
fn baseline_word_frequency() -> io::Result<Vec<(String, usize)>> {
    let mut file = File::open("/tmp/benchmark_data/text_corpus.txt")?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    
    let mut word_counts = HashMap::new();
    
    // Process text with multiple string allocations
    for line in content.lines() {
        let cleaned_line = line
            .to_lowercase()
            .replace(".", "")
            .replace(",", "")
            .replace("!", "")
            .replace("?", "")
            .replace(";", "")
            .replace(":", "");  // Multiple string allocations
        
        let words: Vec<String> = cleaned_line
            .split_whitespace()
            .map(|w| w.to_string())  // Convert each word to owned String
            .collect();
        
        for word in words {
            *word_counts.entry(word.clone()).or_insert(0) += 1;
        }
    }
    
    // Convert to sorted vector (inefficient)
    let mut result: Vec<(String, usize)> = word_counts.into_iter().collect();
    result.sort_by(|a, b| b.1.cmp(&a.1));  // Sort by frequency
    
    Ok(result)
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("baseline_text_tokenization", |b| {
        b.iter(|| black_box(baseline_text_tokenization().unwrap()))
    });
    
    c.bench_function("baseline_json_processing", |b| {
        b.iter(|| black_box(baseline_json_processing().unwrap()))
    });
    
    c.bench_function("baseline_json_filtering", |b| {
        b.iter(|| black_box(baseline_json_filtering().unwrap()))
    });
    
    c.bench_function("baseline_text_parsing", |b| {
        b.iter(|| black_box(baseline_text_parsing().unwrap()))
    });
    
    c.bench_function("baseline_csv_parsing", |b| {
        b.iter(|| black_box(baseline_csv_parsing().unwrap()))
    });
    
    c.bench_function("baseline_word_frequency", |b| {
        b.iter(|| black_box(baseline_word_frequency().unwrap()))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
