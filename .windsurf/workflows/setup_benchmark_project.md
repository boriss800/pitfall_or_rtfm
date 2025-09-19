# Setup Benchmark Project

**Keywords:** setup, initialization, project, benchmark, cargo, infrastructure
**Flags:** `project-setup`, `day-1`, `infrastructure`

## Description
Initialize the Rust benchmark project with proper structure, dependencies, and configuration according to the PRD specifications.

## Steps

### 1. Create Project Structure
```bash
# Create the benchmark project structure
cargo new rust-benchmark-suite --name rust-benchmark-suite
cd rust-benchmark-suite

# Create directory structure as per PRD section 6.1
mkdir -p benches/baseline
mkdir -p benches/optimized  
mkdir -p data/samples
mkdir -p scripts
mkdir -p results/{criterion_reports,flamegraphs,memory_profiles}
```

### 2. Configure Cargo.toml
Create the main `Cargo.toml` with all required dependencies:

```toml
[package]
name = "rust-benchmark-suite"
version = "0.1.0"
edition = "2021"

[dependencies]
rayon = "1.10"
serde_json = "1.0"
ahash = "0.8"
wide = "0.7"
memmap2 = "0.9"
crossbeam = "0.8"

[dev-dependencies]
criterion = { version = "0.5", features = ["html_reports"] }
hyperfine = "1.18"

# Benchmark configurations
[[bench]]
name = "io_workloads_baseline"
harness = false
path = "benches/baseline/io_workloads.rs"

[[bench]]
name = "io_workloads_optimized"
harness = false
path = "benches/optimized/io_workloads.rs"

[[bench]]
name = "parsing_workloads_baseline"
harness = false
path = "benches/baseline/parsing_workloads.rs"

[[bench]]
name = "parsing_workloads_optimized"
harness = false
path = "benches/optimized/parsing_workloads.rs"

[[bench]]
name = "compute_workloads_baseline"
harness = false
path = "benches/baseline/compute_workloads.rs"

[[bench]]
name = "compute_workloads_optimized"
harness = false
path = "benches/optimized/compute_workloads.rs"

[[bench]]
name = "parallel_workloads_baseline"
harness = false
path = "benches/baseline/parallel_workloads.rs"

[[bench]]
name = "parallel_workloads_optimized"
harness = false
path = "benches/optimized/parallel_workloads.rs"

[[bench]]
name = "memory_workloads_baseline"
harness = false
path = "benches/baseline/memory_workloads.rs"

[[bench]]
name = "memory_workloads_optimized"
harness = false
path = "benches/optimized/memory_workloads.rs"

[profile.release]
lto = "fat"
codegen-units = 1
opt-level = 3
panic = "abort"

[profile.dev]
opt-level = 0
debug = true
```

### 3. Create Data Generation Script
Create `data/generate_data.rs`:

```rust
// Data generation for reproducible benchmarks
use std::fs::File;
use std::io::{BufWriter, Write};

fn main() -> std::io::Result<()> {
    println!("Generating benchmark datasets...");
    
    // Generate 1GB text file for I/O benchmarks
    generate_large_text_file("data/large_text.txt", 1_000_000_000)?;
    
    // Generate 100MB CSV file
    generate_csv_file("data/large_data.csv", 1_000_000)?;
    
    // Generate 50MB text corpus
    generate_text_corpus("data/text_corpus.txt", 50_000_000)?;
    
    // Generate JSON records
    generate_json_records("data/json_records.jsonl", 10_000_000)?;
    
    // Generate string pairs for similarity testing
    generate_string_pairs("data/string_pairs.txt", 1_000_000)?;
    
    // Generate numeric data
    generate_numeric_data("data/numeric_data.bin", 100_000_000)?;
    
    println!("Dataset generation complete!");
    Ok(())
}
```

### 4. Create Environment Setup Script
Create `scripts/setup_environment.sh`:

```bash
#!/bin/bash
set -euo pipefail

echo "Setting up benchmark environment..."

# Install required tools
cargo install flamegraph
cargo install hyperfine

# Set CPU governor to performance mode
echo "Setting CPU governor to performance..."
sudo cpupower frequency-set --governor performance

# Disable ASLR for consistent memory layout
echo "Disabling ASLR..."
echo 0 | sudo tee /proc/sys/kernel/randomize_va_space

# Create tmpfs for I/O benchmarks
echo "Creating tmpfs for I/O benchmarks..."
sudo mkdir -p /tmp/benchmark_data
sudo mount -t tmpfs -o size=4G tmpfs /tmp/benchmark_data

# Set CPU affinity for benchmark processes
echo "CPU affinity will be set to cores 0-3 during benchmarks"

echo "Environment setup complete!"
echo "Run 'cargo run --bin generate_data' to create benchmark datasets"
```

### 5. Create Benchmark Execution Script
Create `scripts/run_benchmarks.sh`:

```bash
#!/bin/bash
set -euo pipefail

echo "Starting comprehensive benchmark suite..."

# Ensure environment is set up
./scripts/setup_environment.sh

# Generate test data if not exists
if [ ! -f "data/large_text.txt" ]; then
    echo "Generating benchmark data..."
    cargo run --bin generate_data
fi

# Copy data to tmpfs for I/O benchmarks
echo "Copying data to tmpfs..."
cp data/* /tmp/benchmark_data/

# Run baseline benchmarks with CPU affinity
echo "Running baseline benchmarks..."
taskset -c 0-3 cargo bench --bench io_workloads_baseline 2>&1 | tee results/io_baseline.log
taskset -c 0-3 cargo bench --bench parsing_workloads_baseline 2>&1 | tee results/parsing_baseline.log
taskset -c 0-3 cargo bench --bench compute_workloads_baseline 2>&1 | tee results/compute_baseline.log
taskset -c 0-3 cargo bench --bench parallel_workloads_baseline 2>&1 | tee results/parallel_baseline.log
taskset -c 0-3 cargo bench --bench memory_workloads_baseline 2>&1 | tee results/memory_baseline.log

# Run optimized benchmarks
echo "Running optimized benchmarks..."
taskset -c 0-3 cargo bench --bench io_workloads_optimized 2>&1 | tee results/io_optimized.log
taskset -c 0-3 cargo bench --bench parsing_workloads_optimized 2>&1 | tee results/parsing_optimized.log
taskset -c 0-3 cargo bench --bench compute_workloads_optimized 2>&1 | tee results/compute_optimized.log
taskset -c 0-3 cargo bench --bench parallel_workloads_optimized 2>&1 | tee results/parallel_optimized.log
taskset -c 0-3 cargo bench --bench memory_workloads_optimized 2>&1 | tee results/memory_optimized.log

echo "Benchmark execution complete!"
echo "Results saved in results/ directory"
echo "HTML reports available in target/criterion/"
```

### 6. Create README.md
Create comprehensive project documentation:

```markdown
# Rust Performance Benchmark Suite

## Objective
Scientific benchmarking to disprove the "pitfall theory" by demonstrating measurable performance improvements through Rust optimization techniques.

## Quick Start
1. Run `./scripts/setup_environment.sh` (requires sudo)
2. Run `cargo run --bin generate_data` to create test datasets
3. Run `./scripts/run_benchmarks.sh` to execute all benchmarks
4. View results in `target/criterion/` (HTML reports)

## Project Structure
- `benches/baseline/` - Naive implementations
- `benches/optimized/` - Best-practice implementations  
- `data/` - Test datasets and generation scripts
- `scripts/` - Automation and environment setup
- `results/` - Benchmark outputs and analysis

## Requirements
- Rust 1.70+
- Linux with performance governor support
- 16GB+ RAM
- sudo access for environment setup
```

### 7. Validation Checklist
- [ ] Project structure matches PRD section 6.1
- [ ] All dependencies are correctly specified
- [ ] Benchmark configurations are properly set up
- [ ] Environment setup script handles all requirements
- [ ] Data generation script creates all required datasets
- [ ] README provides clear setup instructions
- [ ] All scripts have proper error handling and logging

## Next Steps
After completing this workflow:
1. Run `/implement-baseline-benchmarks` to create naive implementations
2. Run `/implement-optimized-benchmarks` to create optimized versions
3. Run `/validate-benchmark-setup` to ensure everything works correctly
