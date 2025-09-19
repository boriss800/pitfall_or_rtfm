use std::fs::File;
use std::io::{BufWriter, Write};
use rand::prelude::*;
use rand::distributions::Alphanumeric;

fn main() -> std::io::Result<()> {
    println!("🔧 Generating benchmark datasets according to PRD specifications...");
    
    // Create data directory if it doesn't exist
    std::fs::create_dir_all("data")?;
    
    // Generate 1GB text file for I/O benchmarks
    println!("📄 Generating 1GB text file for I/O benchmarks...");
    generate_large_text_file("data/large_text.txt", 1_000_000_000)?;
    
    // Generate 100MB CSV file with 1M records
    println!("📊 Generating 100MB CSV file with 1M records...");
    generate_csv_file("data/large_data.csv", 1_000_000)?;
    
    // Generate 50MB text corpus for parsing
    println!("📚 Generating 50MB text corpus for parsing benchmarks...");
    generate_text_corpus("data/text_corpus.txt", 50_000_000)?;
    
    // Generate 10M JSON records (~500MB)
    println!("🔗 Generating 10M JSON records for parsing benchmarks...");
    generate_json_records("data/json_records.jsonl", 10_000_000)?;
    
    // Generate 1M string pairs for similarity testing
    println!("🔤 Generating 1M string pairs for similarity benchmarks...");
    generate_string_pairs("data/string_pairs.txt", 1_000_000)?;
    
    // Generate 100M f64 values for numeric benchmarks
    println!("🔢 Generating 100M numeric values for computational benchmarks...");
    generate_numeric_data("data/numeric_data.bin", 100_000_000)?;
    
    // Generate sample datasets for testing
    println!("🧪 Generating smaller sample datasets for testing...");
    generate_sample_datasets()?;
    
    println!("✅ Dataset generation complete!");
    println!("📈 Ready for benchmark execution according to PRD requirements");
    Ok(())
}

fn generate_large_text_file(path: &str, target_size: usize) -> std::io::Result<()> {
    let file = File::create(path)?;
    let mut writer = BufWriter::with_capacity(64 * 1024, file);
    
    let mut rng = thread_rng();
    let words = [
        "performance", "optimization", "rust", "benchmark", "memory", "allocation",
        "simd", "parallel", "rayon", "iterator", "zero", "copy", "buffer", "cache",
        "throughput", "latency", "efficiency", "algorithm", "data", "structure",
        "vector", "string", "processing", "computation", "analysis", "measurement"
    ];
    
    let mut current_size = 0;
    while current_size < target_size {
        // Generate lines with realistic word patterns
        let line_length = rng.gen_range(50..200);
        let mut line = String::with_capacity(line_length);
        
        while line.len() < line_length - 10 {
            let word = words.choose(&mut rng).unwrap();
            if !line.is_empty() {
                line.push(' ');
            }
            line.push_str(word);
        }
        line.push('\n');
        
        writer.write_all(line.as_bytes())?;
        current_size += line.len();
        
        // Progress indicator
        if current_size % (100 * 1024 * 1024) == 0 {
            println!("  Generated {}MB...", current_size / (1024 * 1024));
        }
    }
    
    writer.flush()?;
    Ok(())
}

fn generate_csv_file(path: &str, num_records: usize) -> std::io::Result<()> {
    let file = File::create(path)?;
    let mut writer = BufWriter::with_capacity(64 * 1024, file);
    
    // Write CSV header
    writeln!(writer, "id,name,email,age,city,salary,department")?;
    
    let mut rng = thread_rng();
    let cities = ["New York", "San Francisco", "Seattle", "Austin", "Boston", "Chicago"];
    let departments = ["Engineering", "Marketing", "Sales", "HR", "Finance", "Operations"];
    
    for i in 0..num_records {
        let name: String = (0..rng.gen_range(5..15))
            .map(|_| rng.sample(Alphanumeric) as char)
            .collect();
        
        let email = format!("{}@company.com", name.to_lowercase());
        let age = rng.gen_range(22..65);
        let city = cities.choose(&mut rng).unwrap();
        let salary = rng.gen_range(50000..200000);
        let department = departments.choose(&mut rng).unwrap();
        
        writeln!(writer, "{},{},{},{},{},{},{}", 
                i, name, email, age, city, salary, department)?;
        
        if i % 100_000 == 0 {
            println!("  Generated {} records...", i);
        }
    }
    
    writer.flush()?;
    Ok(())
}

fn generate_text_corpus(path: &str, target_size: usize) -> std::io::Result<()> {
    let file = File::create(path)?;
    let mut writer = BufWriter::with_capacity(64 * 1024, file);
    
    // Generate realistic text content for parsing benchmarks
    let sentences = [
        "The quick brown fox jumps over the lazy dog.",
        "Rust provides zero-cost abstractions for systems programming.",
        "Performance optimization requires careful measurement and analysis.",
        "SIMD instructions can dramatically improve computational throughput.",
        "Memory allocation patterns significantly impact application performance.",
        "Parallel processing enables efficient utilization of multi-core systems.",
        "Benchmarking must account for statistical variance and measurement noise.",
        "Iterator chains provide elegant and efficient data processing pipelines."
    ];
    
    let mut rng = thread_rng();
    let mut current_size = 0;
    
    while current_size < target_size {
        // Generate paragraphs with multiple sentences
        let paragraph_length = rng.gen_range(3..8);
        for _ in 0..paragraph_length {
            let sentence = sentences.choose(&mut rng).unwrap();
            writer.write_all(sentence.as_bytes())?;
            writer.write_all(b" ")?;
            current_size += sentence.len() + 1;
        }
        writer.write_all(b"\n\n")?;
        current_size += 2;
        
        if current_size % (10 * 1024 * 1024) == 0 {
            println!("  Generated {}MB...", current_size / (1024 * 1024));
        }
    }
    
    writer.flush()?;
    Ok(())
}

fn generate_json_records(path: &str, num_records: usize) -> std::io::Result<()> {
    let file = File::create(path)?;
    let mut writer = BufWriter::with_capacity(64 * 1024, file);
    
    let mut rng = thread_rng();
    let event_types = ["login", "logout", "purchase", "view", "click", "search"];
    let user_agents = ["Chrome", "Firefox", "Safari", "Edge"];
    
    for i in 0..num_records {
        let record = serde_json::json!({
            "id": i,
            "timestamp": 1640995200 + rng.gen_range(0..31536000), // Random time in 2022
            "user_id": rng.gen_range(1..100000),
            "event_type": event_types.choose(&mut rng).unwrap(),
            "user_agent": user_agents.choose(&mut rng).unwrap(),
            "ip_address": format!("{}.{}.{}.{}", 
                rng.gen_range(1..255), rng.gen_range(0..255),
                rng.gen_range(0..255), rng.gen_range(1..255)),
            "value": rng.gen_range(1.0..1000.0),
            "metadata": {
                "session_id": format!("sess_{}", rng.gen_range(100000..999999)),
                "page": format!("/page/{}", rng.gen_range(1..100))
            }
        });
        
        writeln!(writer, "{}", record)?;
        
        if i % 1_000_000 == 0 {
            println!("  Generated {} records...", i);
        }
    }
    
    writer.flush()?;
    Ok(())
}

fn generate_string_pairs(path: &str, num_pairs: usize) -> std::io::Result<()> {
    let file = File::create(path)?;
    let mut writer = BufWriter::with_capacity(64 * 1024, file);
    
    let mut rng = thread_rng();
    let first_names = ["John", "Jane", "Michael", "Sarah", "David", "Emily", "Robert", "Lisa"];
    let last_names = ["Smith", "Johnson", "Williams", "Brown", "Jones", "Garcia", "Miller", "Davis"];
    
    for i in 0..num_pairs {
        // Generate realistic name pairs with variations for similarity testing
        let first1 = first_names.choose(&mut rng).unwrap();
        let last1 = last_names.choose(&mut rng).unwrap();
        let name1 = format!("{} {}", first1, last1);
        
        // Create variations: exact match, typos, different names
        let name2 = match rng.gen_range(0..4) {
            0 => name1.clone(), // Exact match
            1 => { // Minor typo
                let mut chars: Vec<char> = name1.chars().collect();
                if chars.len() > 2 {
                    let idx = rng.gen_range(1..chars.len()-1);
                    chars[idx] = rng.sample(Alphanumeric) as char;
                }
                chars.into_iter().collect()
            },
            2 => { // Same first name, different last
                let last2 = last_names.choose(&mut rng).unwrap();
                format!("{} {}", first1, last2)
            },
            _ => { // Completely different name
                let first2 = first_names.choose(&mut rng).unwrap();
                let last2 = last_names.choose(&mut rng).unwrap();
                format!("{} {}", first2, last2)
            }
        };
        
        writeln!(writer, "{}\t{}", name1, name2)?;
        
        if i % 100_000 == 0 {
            println!("  Generated {} pairs...", i);
        }
    }
    
    writer.flush()?;
    Ok(())
}

fn generate_numeric_data(path: &str, num_values: usize) -> std::io::Result<()> {
    let file = File::create(path)?;
    let mut writer = BufWriter::with_capacity(64 * 1024, file);
    
    let mut rng = thread_rng();
    
    // Generate realistic numeric data with various distributions
    for i in 0..num_values {
        let value = match i % 4 {
            0 => rng.gen_range(-1000.0..1000.0), // Uniform distribution
            1 => rng.sample::<f64, _>(rand_distr::StandardNormal) * 100.0, // Normal distribution
            2 => rng.sample::<f64, _>(rand_distr::Exp::new(0.01).unwrap()), // Exponential
            _ => (i as f64 * 0.1) + rng.gen_range(-10.0..10.0), // Linear with noise
        };
        
        writer.write_all(&value.to_le_bytes())?;
        
        if i % 10_000_000 == 0 {
            println!("  Generated {} values...", i);
        }
    }
    
    writer.flush()?;
    Ok(())
}

fn generate_sample_datasets() -> std::io::Result<()> {
    std::fs::create_dir_all("data/samples")?;
    
    // Small versions for testing and validation
    generate_large_text_file("data/samples/small_text.txt", 1_000)?;
    generate_csv_file("data/samples/small_data.csv", 100)?;
    generate_text_corpus("data/samples/small_corpus.txt", 5_000)?;
    generate_json_records("data/samples/small_records.jsonl", 1_000)?;
    generate_string_pairs("data/samples/small_pairs.txt", 1_000)?;
    generate_numeric_data("data/samples/small_numeric.bin", 10_000)?;
    
    Ok(())
}
