#!/bin/bash
set -euo pipefail

echo "🔬 Starting comprehensive benchmark correctness validation..."
echo "📋 This ensures baseline and optimized implementations produce identical results"

# Ensure test data exists
if [[ ! -f "/tmp/benchmark_data/samples/small_corpus.txt" ]]; then
    echo "📦 Generating test datasets..."
    cargo run --bin generate_data
fi

# Create small test datasets if they don't exist
echo "🧪 Ensuring small test datasets exist..."
mkdir -p /tmp/benchmark_data/samples

# Create small corpus for testing if it doesn't exist
if [[ ! -f "/tmp/benchmark_data/samples/small_corpus.txt" ]]; then
    echo "Creating small test corpus..."
    head -n 100 /tmp/benchmark_data/text_corpus.txt > /tmp/benchmark_data/samples/small_corpus.txt 2>/dev/null || {
        echo "performance optimization rust benchmark memory allocation" > /tmp/benchmark_data/samples/small_corpus.txt
        echo "simd parallel rayon iterator zero copy buffer cache" >> /tmp/benchmark_data/samples/small_corpus.txt
        echo "throughput latency efficiency algorithm data structure" >> /tmp/benchmark_data/samples/small_corpus.txt
        echo "vector string processing computation analysis measurement" >> /tmp/benchmark_data/samples/small_corpus.txt
    }
fi

echo "✅ Test datasets ready"

# Run correctness validation tests
echo ""
echo "🧪 Running correctness validation tests..."

# Test compilation first
echo "  📝 Checking test compilation..."
cargo test --no-run --test correctness_validation

# Run I/O workload validation
echo "  💾 Validating I/O workloads..."
cargo test --test correctness_validation io_workload_validation -- --nocapture

# Run parsing workload validation
echo "  📄 Validating parsing workloads..."
cargo test --test correctness_validation parsing_workload_validation -- --nocapture

# Run computational workload validation
echo "  🧮 Validating computational workloads..."
cargo test --test correctness_validation computational_workload_validation -- --nocapture

# Run memory workload validation
echo "  🧠 Validating memory workloads..."
cargo test --test correctness_validation memory_workload_validation -- --nocapture

# Run comprehensive validation
echo "  🔍 Running comprehensive validation..."
cargo test --test correctness_validation run_comprehensive_validation -- --nocapture

echo ""
echo "✅ All correctness validations passed!"
echo ""
echo "📊 Validation Summary:"
echo "  ✅ I/O workloads: Baseline and optimized produce identical results"
echo "  ✅ Parsing workloads: Word frequency and text processing validated"
echo "  ✅ Computational workloads: Jaro-Winkler and prime calculation validated"
echo "  ✅ Memory workloads: Collection pipelines and string building validated"
echo ""
echo "🎯 Scientific Validity Confirmed:"
echo "  • Optimizations preserve correctness"
echo "  • No algorithmic changes that affect results"
echo "  • Ready for performance measurement"
echo ""
echo "📋 Next Steps:"
echo "  1. Run './scripts/run_benchmarks.sh' for performance measurement"
echo "  2. Generate statistical analysis of results"
echo "  3. Create final report with hypothesis test results"
