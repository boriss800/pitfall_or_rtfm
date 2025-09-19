# Execute Comprehensive Benchmarks

**Keywords:** benchmarks, execution, performance, measurement, statistical, analysis
**Flags:** `day-4`, `performance-measurement`, `statistical-analysis`

## Description
Execute the complete benchmark suite with proper statistical rigor, environment controls, and comprehensive data collection for the final performance analysis.

## Critical Requirements
- **Statistical significance** with 100+ iterations per benchmark
- **Environment isolation** with CPU pinning and performance governor
- **Comprehensive profiling** with flamegraphs and memory analysis
- **Data collection** in multiple formats for analysis
- **Reproducible results** across multiple independent runs

## Steps

### 1. Pre-Benchmark Environment Setup
```bash
#!/bin/bash
# scripts/setup_benchmark_environment.sh

echo "Configuring system for benchmarking..."

# Set CPU governor to performance mode
sudo cpupower frequency-set --governor performance

# Disable CPU frequency scaling and turbo boost
echo 1 | sudo tee /sys/devices/system/cpu/intel_pstate/no_turbo
for cpu in /sys/devices/system/cpu/cpu*/cpufreq/scaling_governor; do
    echo performance | sudo tee $cpu
done

# Disable ASLR for consistent memory layout
echo 0 | sudo tee /proc/sys/kernel/randomize_va_space

# Set up tmpfs for I/O benchmarks
sudo mkdir -p /tmp/benchmark_data
sudo mount -t tmpfs -o size=8G tmpfs /tmp/benchmark_data

# Copy datasets to tmpfs
cp data/* /tmp/benchmark_data/

# Clear system caches
sync
echo 3 | sudo tee /proc/sys/vm/drop_caches

echo "Environment ready for benchmarking"
```

### 2. Execute Baseline Benchmarks
```bash
#!/bin/bash
# scripts/run_baseline_benchmarks.sh

echo "Executing baseline benchmarks with statistical rigor..."

# Pin to specific CPU cores for consistency
TASKSET="taskset -c 0-3"

# Run each benchmark category with comprehensive settings
$TASKSET cargo bench --bench io_workloads_baseline \
    -- --sample-size 100 --measurement-time 30 \
    --output-format json > results/baseline_io.json

$TASKSET cargo bench --bench parsing_workloads_baseline \
    -- --sample-size 100 --measurement-time 30 \
    --output-format json > results/baseline_parsing.json

$TASKSET cargo bench --bench compute_workloads_baseline \
    -- --sample-size 100 --measurement-time 30 \
    --output-format json > results/baseline_compute.json

$TASKSET cargo bench --bench parallel_workloads_baseline \
    -- --sample-size 100 --measurement-time 30 \
    --output-format json > results/baseline_parallel.json

$TASKSET cargo bench --bench memory_workloads_baseline \
    -- --sample-size 100 --measurement-time 30 \
    --output-format json > results/baseline_memory.json

echo "Baseline benchmarks complete"
```

### 3. Execute Optimized Benchmarks
```bash
#!/bin/bash
# scripts/run_optimized_benchmarks.sh

echo "Executing optimized benchmarks..."

TASKSET="taskset -c 0-3"

# Run optimized benchmarks with same statistical settings
$TASKSET cargo bench --bench io_workloads_optimized \
    -- --sample-size 100 --measurement-time 30 \
    --output-format json > results/optimized_io.json

$TASKSET cargo bench --bench parsing_workloads_optimized \
    -- --sample-size 100 --measurement-time 30 \
    --output-format json > results/optimized_parsing.json

$TASKSET cargo bench --bench compute_workloads_optimized \
    -- --sample-size 100 --measurement-time 30 \
    --output-format json > results/optimized_compute.json

$TASKSET cargo bench --bench parallel_workloads_optimized \
    -- --sample-size 100 --measurement-time 30 \
    --output-format json > results/optimized_parallel.json

$TASKSET cargo bench --bench memory_workloads_optimized \
    -- --sample-size 100 --measurement-time 30 \
    --output-format json > results/optimized_memory.json

echo "Optimized benchmarks complete"
```

### 4. Generate Performance Profiles
```bash
#!/bin/bash
# scripts/generate_profiles.sh

echo "Generating performance profiles..."

# Generate flamegraphs for baseline implementations
cargo flamegraph --bench io_workloads_baseline --output results/flamegraphs/baseline_io.svg
cargo flamegraph --bench compute_workloads_baseline --output results/flamegraphs/baseline_compute.svg

# Generate flamegraphs for optimized implementations  
cargo flamegraph --bench io_workloads_optimized --output results/flamegraphs/optimized_io.svg
cargo flamegraph --bench compute_workloads_optimized --output results/flamegraphs/optimized_compute.svg

# Memory profiling with heaptrack
heaptrack cargo bench --bench memory_workloads_baseline -- --test
mv heaptrack.*.gz results/memory_profiles/baseline_memory.gz

heaptrack cargo bench --bench memory_workloads_optimized -- --test  
mv heaptrack.*.gz results/memory_profiles/optimized_memory.gz

echo "Performance profiles generated"
```

### 5. Statistical Analysis Script
Create `scripts/analyze_results.py`:

```python
#!/usr/bin/env python3
import json
import numpy as np
from scipy import stats
import matplotlib.pyplot as plt
import pandas as pd

def load_benchmark_results(baseline_file, optimized_file):
    """Load and parse benchmark results"""
    with open(baseline_file) as f:
        baseline = json.load(f)
    with open(optimized_file) as f:
        optimized = json.load(f)
    return baseline, optimized

def calculate_statistics(baseline_times, optimized_times):
    """Calculate comprehensive statistics"""
    # Basic statistics
    baseline_mean = np.mean(baseline_times)
    optimized_mean = np.mean(optimized_times)
    improvement_ratio = baseline_mean / optimized_mean
    
    # Statistical significance test
    t_stat, p_value = stats.ttest_ind(baseline_times, optimized_times)
    
    # Effect size (Cohen's d)
    pooled_std = np.sqrt(((len(baseline_times) - 1) * np.var(baseline_times, ddof=1) + 
                         (len(optimized_times) - 1) * np.var(optimized_times, ddof=1)) / 
                        (len(baseline_times) + len(optimized_times) - 2))
    cohens_d = (baseline_mean - optimized_mean) / pooled_std
    
    # Confidence intervals
    baseline_ci = stats.t.interval(0.95, len(baseline_times)-1, 
                                  loc=baseline_mean, 
                                  scale=stats.sem(baseline_times))
    optimized_ci = stats.t.interval(0.95, len(optimized_times)-1,
                                   loc=optimized_mean,
                                   scale=stats.sem(optimized_times))
    
    return {
        'baseline_mean': baseline_mean,
        'optimized_mean': optimized_mean,
        'improvement_ratio': improvement_ratio,
        'p_value': p_value,
        'cohens_d': cohens_d,
        'baseline_ci': baseline_ci,
        'optimized_ci': optimized_ci,
        'significant': p_value < 0.05 and cohens_d > 0.8
    }

def generate_report(results):
    """Generate comprehensive analysis report"""
    report = []
    report.append("# Rust Performance Benchmark Results\n")
    
    significant_improvements = 0
    total_benchmarks = len(results)
    
    for benchmark_name, stats in results.items():
        report.append(f"## {benchmark_name}")
        report.append(f"- **Improvement Ratio**: {stats['improvement_ratio']:.2f}x")
        report.append(f"- **P-value**: {stats['p_value']:.2e}")
        report.append(f"- **Effect Size (Cohen's d)**: {stats['cohens_d']:.3f}")
        report.append(f"- **Statistically Significant**: {'Yes' if stats['significant'] else 'No'}")
        
        if stats['significant']:
            significant_improvements += 1
        
        report.append("")
    
    # Hypothesis test results
    report.append("## Hypothesis Test Results")
    report.append(f"- **Significant improvements**: {significant_improvements}/{total_benchmarks}")
    report.append(f"- **Success rate**: {significant_improvements/total_benchmarks*100:.1f}%")
    
    hypothesis_result = "REJECTED" if significant_improvements >= 3 else "FAILED TO REJECT"
    report.append(f"- **Null Hypothesis (H₀)**: {hypothesis_result}")
    
    return "\n".join(report)

if __name__ == "__main__":
    # Analyze all benchmark categories
    categories = ['io', 'parsing', 'compute', 'parallel', 'memory']
    all_results = {}
    
    for category in categories:
        baseline_file = f"results/baseline_{category}.json"
        optimized_file = f"results/optimized_{category}.json"
        
        try:
            baseline, optimized = load_benchmark_results(baseline_file, optimized_file)
            # Extract timing data and calculate statistics
            # (Implementation details depend on criterion output format)
            stats = calculate_statistics(baseline_data, optimized_data)
            all_results[category] = stats
        except FileNotFoundError:
            print(f"Warning: Results not found for {category}")
    
    # Generate and save report
    report = generate_report(all_results)
    with open("results/statistical_analysis.md", "w") as f:
        f.write(report)
    
    print("Statistical analysis complete!")
```

## Validation Checklist
- [ ] Environment properly configured (CPU governor, ASLR, tmpfs)
- [ ] CPU affinity pinned to specific cores
- [ ] Minimum 100 iterations per benchmark
- [ ] Statistical significance calculated (p < 0.05)
- [ ] Effect sizes measured (Cohen's d > 0.8)
- [ ] Confidence intervals computed
- [ ] Flamegraphs generated for performance analysis
- [ ] Memory profiles captured
- [ ] Results exported in multiple formats
- [ ] Multiple independent runs completed

## Success Criteria
- **≥3 categories** show ≥2x improvement with statistical significance
- **P-values < 0.05** for all significant improvements
- **Cohen's d > 0.8** for large effect sizes
- **Reproducible results** across independent runs
- **Complete profiling data** for analysis

## Next Steps
After benchmark execution:
1. Run `/generate-performance-analysis` to create comprehensive analysis
2. Run `/create-final-report` to compile deliverable documentation
3. Run `/validate-hypothesis-results` to confirm scientific conclusions
