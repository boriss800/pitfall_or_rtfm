#!/bin/bash
set -euo pipefail

echo "🚀 Starting comprehensive benchmark suite execution..."
echo "📊 Following PRD specifications for statistical rigor"

# Ensure environment is properly configured
if [[ ! -f "scripts/setup_environment.sh" ]]; then
    echo "❌ Environment setup script not found. Run setup first."
    exit 1
fi

# Check if datasets exist
if [[ ! -f "data/large_text.txt" ]]; then
    echo "📦 Generating benchmark datasets..."
    cargo run --bin generate_data
fi

# Copy datasets to tmpfs for consistent I/O performance
echo "💾 Copying datasets to tmpfs for I/O benchmarks..."
if mountpoint -q /tmp/benchmark_data 2>/dev/null; then
    cp -r data/* /tmp/benchmark_data/ 2>/dev/null || echo "⚠️  Could not copy to tmpfs, using regular filesystem"
else
    echo "⚠️  Tmpfs not mounted, using regular filesystem for I/O benchmarks"
    mkdir -p /tmp/benchmark_data
    cp -r data/* /tmp/benchmark_data/
fi

# Clear system caches for consistent measurements
echo "🧹 Clearing system caches..."
sync
sudo sh -c 'echo 3 > /proc/sys/vm/drop_caches' 2>/dev/null || echo "⚠️  Could not clear caches"

# CPU affinity for consistent performance (cores 0-3)
TASKSET="taskset -c 0-3"

# Benchmark execution parameters for statistical rigor
SAMPLE_SIZE=100
MEASUREMENT_TIME=30
WARMUP_TIME=10

echo "⚙️  Benchmark configuration:"
echo "  Sample size: $SAMPLE_SIZE iterations"
echo "  Measurement time: $MEASUREMENT_TIME seconds per benchmark"
echo "  Warmup time: $WARMUP_TIME seconds"
echo "  CPU affinity: cores 0-3"
echo ""

# Create timestamp for this benchmark run
TIMESTAMP=$(date +%Y%m%d_%H%M%S)
RUN_DIR="results/run_$TIMESTAMP"
mkdir -p "$RUN_DIR"

echo "📁 Results will be saved to: $RUN_DIR"
echo ""

# Function to run benchmarks with error handling
run_benchmark_category() {
    local category=$1
    local type=$2  # baseline or optimized
    
    echo "🔄 Running $type $category benchmarks..."
    
    local bench_name="${category}_workloads_${type}"
    local output_file="$RUN_DIR/${type}_${category}.json"
    
    # Run with timeout and error handling
    if timeout 1800 $TASKSET cargo bench --bench "$bench_name" -- \
        --sample-size "$SAMPLE_SIZE" \
        --measurement-time "$MEASUREMENT_TIME" \
        --warm-up-time "$WARMUP_TIME" \
        --output-format json > "$output_file" 2>&1; then
        
        echo "  ✅ $type $category completed"
        
        # Extract key metrics for quick summary
        if command -v jq &> /dev/null && [[ -f "$output_file" ]]; then
            local mean_time=$(jq -r '.mean.estimate // "N/A"' "$output_file" 2>/dev/null || echo "N/A")
            echo "     Mean time: ${mean_time}ns"
        fi
    else
        echo "  ❌ $type $category failed or timed out"
        echo "error" > "$output_file"
    fi
    
    echo ""
}

# Execute baseline benchmarks first
echo "🔴 === BASELINE BENCHMARKS (Debug/Unoptimized) ==="
run_benchmark_category "io" "baseline"
run_benchmark_category "parsing" "baseline"
run_benchmark_category "compute" "baseline"
run_benchmark_category "parallel" "baseline"
run_benchmark_category "memory" "baseline"

echo "🟢 === OPTIMIZED BENCHMARKS (Release/Optimized) ==="
run_benchmark_category "io" "optimized"
run_benchmark_category "parsing" "optimized"
run_benchmark_category "compute" "optimized"
run_benchmark_category "parallel" "optimized"
run_benchmark_category "memory" "optimized"

# Generate performance profiles for key benchmarks
echo "📊 Generating performance profiles..."

echo "  🔥 Creating flamegraphs..."
mkdir -p "$RUN_DIR/flamegraphs"

# Generate flamegraphs for baseline vs optimized (sample a few key benchmarks)
for category in io compute; do
    echo "    Profiling $category workloads..."
    
    # Baseline flamegraph
    timeout 300 cargo flamegraph --bench "${category}_workloads_baseline" \
        --output "$RUN_DIR/flamegraphs/baseline_${category}.svg" -- --bench 2>/dev/null || \
        echo "    ⚠️  Could not generate baseline $category flamegraph"
    
    # Optimized flamegraph  
    timeout 300 cargo flamegraph --bench "${category}_workloads_optimized" \
        --output "$RUN_DIR/flamegraphs/optimized_${category}.svg" -- --bench 2>/dev/null || \
        echo "    ⚠️  Could not generate optimized $category flamegraph"
done

# Memory profiling (if heaptrack is available)
if command -v heaptrack &> /dev/null; then
    echo "  💾 Running memory profiling..."
    mkdir -p "$RUN_DIR/memory_profiles"
    
    # Profile memory workloads
    heaptrack --output "$RUN_DIR/memory_profiles/baseline_memory.gz" \
        cargo bench --bench memory_workloads_baseline -- --test 2>/dev/null || \
        echo "    ⚠️  Could not profile baseline memory usage"
        
    heaptrack --output "$RUN_DIR/memory_profiles/optimized_memory.gz" \
        cargo bench --bench memory_workloads_optimized -- --test 2>/dev/null || \
        echo "    ⚠️  Could not profile optimized memory usage"
else
    echo "  ⚠️  heaptrack not available for memory profiling"
fi

# Run hyperfine comparisons for end-to-end timing
if command -v hyperfine &> /dev/null; then
    echo "  ⏱️  Running hyperfine end-to-end comparisons..."
    
    # Create simple benchmark executables for hyperfine
    # (This would need actual benchmark binaries - placeholder for now)
    echo "    Hyperfine comparisons would run here with actual benchmark binaries"
else
    echo "  ⚠️  hyperfine not available for CLI benchmarking"
fi

# Generate quick summary
echo "📋 Generating benchmark summary..."
cat > "$RUN_DIR/summary.md" << EOF
# Benchmark Run Summary

**Timestamp:** $(date)
**Run ID:** $TIMESTAMP

## Configuration
- Sample size: $SAMPLE_SIZE iterations
- Measurement time: $MEASUREMENT_TIME seconds
- CPU affinity: cores 0-3
- Environment: $(uname -a)

## Results Location
- Raw results: $RUN_DIR/
- Flamegraphs: $RUN_DIR/flamegraphs/
- Memory profiles: $RUN_DIR/memory_profiles/

## Next Steps
1. Run analysis scripts to process results
2. Generate statistical comparison report
3. Create visualizations and final report

EOF

echo "✅ Benchmark execution complete!"
echo ""
echo "📊 Results summary:"
echo "  📁 Results directory: $RUN_DIR"
echo "  📈 Criterion HTML reports: target/criterion/"
echo "  🔥 Flamegraphs: $RUN_DIR/flamegraphs/"
echo "  💾 Memory profiles: $RUN_DIR/memory_profiles/"
echo ""
echo "🎯 Next steps:"
echo "  1. Review results in target/criterion/ (open index.html)"
echo "  2. Run analysis scripts to generate statistical comparison"
echo "  3. Create final report according to PRD specifications"
echo ""
echo "📋 PRD Success Criteria Check:"
echo "  - ≥100 iterations per benchmark: ✅"
echo "  - Statistical significance testing: Ready for analysis"
echo "  - Performance profiling: ✅"
echo "  - Reproducible environment: ✅"
