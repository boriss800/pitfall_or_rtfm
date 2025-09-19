use std::fs::File;
use std::io::{self, BufRead, BufReader, BufWriter, Write};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

/// Optimized large file processing with buffering and streaming
/// Uses BufReader for efficient I/O and streaming processing without loading entire file
fn optimized_large_file_processing() -> io::Result<usize> {
    let file = File::open("/tmp/benchmark_data/large_text.txt")?;
    let reader = BufReader::with_capacity(64 * 1024, file);
    
    // Stream processing - no need to load entire file into memory
    let word_count = reader
        .lines()
        .map(|line| {
            line.unwrap()
                .split_whitespace()
                .count()  // No intermediate collection needed
        })
        .sum();
    
    Ok(word_count)
}

/// Optimized CSV transformation with buffered I/O and pre-allocation
fn optimized_csv_transformation() -> io::Result<()> {
    let file = File::open("/tmp/benchmark_data/large_data.csv")?;
    let reader = BufReader::with_capacity(64 * 1024, file);
    
    let output_file = File::create("/tmp/benchmark_data/output_optimized.csv")?;
    let mut writer = BufWriter::with_capacity(64 * 1024, output_file);
    
    for line in reader.lines() {
        let line = line?;
        let mut output_line = String::with_capacity(line.len() * 2); // Pre-allocate capacity
        
        for (i, field) in line.split(',').enumerate() {
            if i > 0 {
                output_line.push(',');
            }
            // Efficient uppercase conversion without intermediate allocations
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

/// Optimized file copying with large buffer and efficient I/O
fn optimized_file_copy() -> io::Result<()> {
    let mut input = File::open("/tmp/benchmark_data/large_data.csv")?;
    let mut output = File::create("/tmp/benchmark_data/copy_optimized.csv")?;
    
    // Use large buffer for efficient copying
    let mut buffer = vec![0u8; 64 * 1024];
    
    loop {
        let bytes_read = std::io::Read::read(&mut input, &mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        output.write_all(&buffer[..bytes_read])?;
    }
    
    Ok(())
}

/// Optimized line counting with streaming and no allocations
fn optimized_line_counting() -> io::Result<usize> {
    let file = File::open("/tmp/benchmark_data/text_corpus.txt")?;
    let reader = BufReader::with_capacity(64 * 1024, file);
    
    // Count lines without allocating strings
    let line_count = reader.lines().count();
    
    Ok(line_count)
}

/// Optimized text search with streaming and efficient string operations
fn optimized_text_search() -> io::Result<usize> {
    let file = File::open("/tmp/benchmark_data/text_corpus.txt")?;
    let reader = BufReader::with_capacity(64 * 1024, file);
    
    let search_terms = ["performance", "optimization", "rust", "benchmark"];
    let mut total_matches = 0;
    
    for line in reader.lines() {
        let line = line?;
        let line_lower = line.to_lowercase();
        
        // Efficient search using iterator and avoiding unnecessary collections
        for &term in &search_terms {
            total_matches += line_lower
                .split_whitespace()
                .filter(|&word| word == term)
                .count();
        }
    }
    
    Ok(total_matches)
}

/// Optimized memory-mapped file processing for large files
fn optimized_mmap_processing() -> io::Result<usize> {
    use memmap2::MmapOptions;
    
    let file = File::open("/tmp/benchmark_data/large_text.txt")?;
    let mmap = unsafe { MmapOptions::new().map(&file)? };
    
    // Process memory-mapped file efficiently
    let content = std::str::from_utf8(&mmap).map_err(|e| {
        io::Error::new(io::ErrorKind::InvalidData, e)
    })?;
    
    // Count words using zero-copy string processing
    let word_count = content
        .lines()
        .map(|line| line.split_whitespace().count())
        .sum();
    
    Ok(word_count)
}

/// Optimized parallel file processing using multiple threads
fn optimized_parallel_file_processing() -> io::Result<usize> {
    use rayon::prelude::*;
    
    let file = File::open("/tmp/benchmark_data/large_text.txt")?;
    let reader = BufReader::with_capacity(64 * 1024, file);
    
    // Collect lines first, then process in parallel
    let lines: Vec<String> = reader.lines().collect::<Result<Vec<_>, _>>()?;
    
    // Process lines in parallel
    let word_count: usize = lines
        .par_iter()
        .map(|line| line.split_whitespace().count())
        .sum();
    
    Ok(word_count)
}

/// Optimized streaming JSON processing with minimal allocations
fn optimized_streaming_json() -> io::Result<usize> {
    let file = File::open("/tmp/benchmark_data/json_records.jsonl")?;
    let reader = BufReader::with_capacity(64 * 1024, file);
    
    let mut count = 0;
    
    for line in reader.lines() {
        let line = line?;
        
        // Quick validation without full parsing for counting
        if line.starts_with('{') && line.ends_with('}') {
            count += 1;
        }
    }
    
    Ok(count)
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("optimized_large_file_processing", |b| {
        b.iter(|| black_box(optimized_large_file_processing().unwrap()))
    });
    
    c.bench_function("optimized_csv_transformation", |b| {
        b.iter(|| black_box(optimized_csv_transformation().unwrap()))
    });
    
    c.bench_function("optimized_file_copy", |b| {
        b.iter(|| black_box(optimized_file_copy().unwrap()))
    });
    
    c.bench_function("optimized_line_counting", |b| {
        b.iter(|| black_box(optimized_line_counting().unwrap()))
    });
    
    c.bench_function("optimized_text_search", |b| {
        b.iter(|| black_box(optimized_text_search().unwrap()))
    });
    
    c.bench_function("optimized_mmap_processing", |b| {
        b.iter(|| black_box(optimized_mmap_processing().unwrap()))
    });
    
    c.bench_function("optimized_parallel_file_processing", |b| {
        b.iter(|| black_box(optimized_parallel_file_processing().unwrap()))
    });
    
    c.bench_function("optimized_streaming_json", |b| {
        b.iter(|| black_box(optimized_streaming_json().unwrap()))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
