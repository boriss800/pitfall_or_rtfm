use std::fs::File;
use std::io::{self, Read, Write};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

/// Baseline (naive) large file processing - loads entire 1GB file into memory
/// This represents typical developer approach without optimization considerations
fn baseline_large_file_processing() -> io::Result<usize> {
    // Naive approach: load entire file into memory at once
    let mut file = File::open("/tmp/benchmark_data/large_text.txt")?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;  // Load entire 1GB file into memory
    
    let mut word_count = 0;
    
    // Inefficient: creates new Vec for each line
    for line in content.lines() {
        let words: Vec<&str> = line.split_whitespace().collect();  // Unnecessary collection
        word_count += words.len();
    }
    
    Ok(word_count)
}

/// Baseline CSV transformation with inefficient string concatenation
/// Demonstrates typical string building patterns without pre-allocation
fn baseline_csv_transformation() -> io::Result<()> {
    // Load entire file into memory (naive approach)
    let mut file = File::open("/tmp/benchmark_data/large_data.csv")?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    
    let mut output = String::new();  // No capacity pre-allocation
    
    for line in content.lines() {
        let fields: Vec<&str> = line.split(',').collect();  // Unnecessary collection
        
        for (i, field) in fields.iter().enumerate() {
            if i > 0 {
                output = output + ",";  // Inefficient string concatenation
            }
            // Convert to uppercase with string concatenation
            output = output + &field.to_uppercase();
        }
        output = output + "\n";  // More inefficient concatenation
    }
    
    // Write output without buffering
    let mut output_file = File::create("/tmp/benchmark_data/output_baseline.csv")?;
    output_file.write_all(output.as_bytes())?;
    
    Ok(())
}

/// Baseline file copying with byte-by-byte reading (extremely inefficient)
fn baseline_file_copy() -> io::Result<()> {
    let mut input = File::open("/tmp/benchmark_data/large_data.csv")?;
    let mut output = File::create("/tmp/benchmark_data/copy_baseline.csv")?;
    
    // Extremely naive: read one byte at a time
    let mut buffer = [0u8; 1];
    loop {
        match input.read(&mut buffer)? {
            0 => break,  // EOF
            _ => {
                output.write_all(&buffer)?;  // Write one byte at a time
            }
        }
    }
    
    Ok(())
}

/// Baseline line counting with string allocation for each line
fn baseline_line_counting() -> io::Result<usize> {
    let mut file = File::open("/tmp/benchmark_data/text_corpus.txt")?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    
    let mut line_count = 0;
    
    // Inefficient: convert each line to owned String
    for line in content.lines() {
        let _owned_line = line.to_string();  // Unnecessary allocation
        line_count += 1;
    }
    
    Ok(line_count)
}

/// Baseline text search with repeated string allocations
fn baseline_text_search() -> io::Result<usize> {
    let mut file = File::open("/tmp/benchmark_data/text_corpus.txt")?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    
    let search_terms = ["performance", "optimization", "rust", "benchmark"];
    let mut total_matches = 0;
    
    // Inefficient: search each term separately with string allocations
    for term in &search_terms {
        for line in content.lines() {
            let line_lower = line.to_lowercase();  // Allocate new string for each line
            let term_lower = term.to_lowercase();  // Allocate new string for each term
            
            // Count occurrences with inefficient splitting
            let words: Vec<&str> = line_lower.split_whitespace().collect();
            for word in words {
                if word == term_lower {
                    total_matches += 1;
                }
            }
        }
    }
    
    Ok(total_matches)
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("baseline_large_file_processing", |b| {
        b.iter(|| black_box(baseline_large_file_processing().unwrap()))
    });
    
    c.bench_function("baseline_csv_transformation", |b| {
        b.iter(|| black_box(baseline_csv_transformation().unwrap()))
    });
    
    c.bench_function("baseline_file_copy", |b| {
        b.iter(|| black_box(baseline_file_copy().unwrap()))
    });
    
    c.bench_function("baseline_line_counting", |b| {
        b.iter(|| black_box(baseline_line_counting().unwrap()))
    });
    
    c.bench_function("baseline_text_search", |b| {
        b.iter(|| black_box(baseline_text_search().unwrap()))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
