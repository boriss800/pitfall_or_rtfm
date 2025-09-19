# PRD Steering Rules

**Keywords:** benchmark, performance, rust, optimization, scientific, hypothesis, prd
**Flags:** `always-active`, `project-critical`, `quality-gate`

## Project Objective Alignment

<project_mission>
- **Primary Mission**: This project exists to scientifically disprove the "pitfall theory" through rigorous Rust performance benchmarking
- **Hypothesis Focus**: Always work toward rejecting H₀ (null hypothesis) that optimized Rust provides <20% improvement
- **Target Goal**: Demonstrate ≥2x performance improvement in at least 3 out of 5 benchmark categories
- **Scientific Rigor**: All implementations must follow the statistical requirements defined in the PRD (95% confidence, Cohen's d > 0.8)
</project_mission>

## Implementation Standards

### Baseline Implementation Requirements
<baseline_standards>
- Use debug builds with no optimizations (`cargo build`)
- Implement direct unbuffered I/O operations (`File::open()` → `read_to_string()`)
- Use `Read::lines()` with frequent allocations
- Employ index-based loops with bounds checking (`for i in 0..len`)
- Include frequent `.collect()` usage in pipelines
- Use liberal `.clone()` calls instead of borrowing
- Avoid SIMD, parallelism, and profiling guidance
- Use string concatenation with `+` operator
- Create new `Vec` for each intermediate step
</baseline_standards>

### Optimized Implementation Requirements
<optimized_standards>
- Use release builds with LTO + `opt-level=3`
- Implement buffered I/O (`BufReader::with_capacity(64*1024)`)
- Pre-allocate and reuse string buffers (`String::with_capacity()`)
- Use iterator chains and slice operations (`.iter().map().filter()`)
- Employ zero-copy borrowing instead of cloning (`&str` vs `String`)
- Include SIMD intrinsics (`std::simd`, `wide` crate) where applicable
- Use Rayon for embarrassingly parallel workloads (`par_iter()`)
- Apply profile-guided optimization with flamegraphs
- Use efficient string building and streaming iterators
</optimized_standards>

## Benchmark Workload Compliance

### Required Workloads (Must Implement All)
<workload_requirements>
1. **I/O Bound**: 1GB file processing, 100MB CSV transformation
2. **Parsing**: 50MB text tokenization, 10M JSON records processing  
3. **Computational**: 1M string similarity comparisons, 100M numeric aggregation
4. **Parallelism**: Multi-file word count, 1000x1000 matrix operations
5. **Memory**: 10M integer stream processing with allocation patterns
</workload_requirements>

### Dataset Requirements
<dataset_standards>
- Use realistic data sources (Wikipedia dumps, Project Gutenberg, sensor data)
- Maintain consistent dataset sizes across baseline and optimized implementations
- Generate reproducible test data with provided scripts
- Store datasets in `data/` directory with generation scripts
</dataset_standards>

## Quality Assurance Rules

### Statistical Validation
<statistical_requirements>
- Minimum 100 iterations per benchmark
- Coefficient of variation < 5% for stable results
- Apply Bonferroni correction for multiple comparisons
- Results must be reproducible across 3 independent runs
- Use criterion.rs with proper confidence intervals
- Report p-values and effect sizes for all comparisons
</statistical_requirements>

### Code Quality
<code_quality_gates>
- All benchmarks must use `criterion::black_box` to prevent optimization
- Include proper error handling in all implementations
- Document all optimization decisions transparently
- Use consistent naming conventions: `baseline_*` and `optimized_*`
- Add inline comments explaining naive vs optimized approaches
- Include flamegraph generation for all optimized implementations
</code_quality_gates>

### Environment Controls
<environment_requirements>
- Pin CPU affinity with `taskset -c 0-3`
- Disable CPU frequency scaling (`performance` governor)
- Use tmpfs for I/O benchmarks to eliminate disk variance
- Document all environment setup steps in `scripts/setup_environment.sh`
- Disable ASLR for consistent memory layout
- Use isolated CPU cores, disable hyperthreading
</environment_requirements>

## Anti-Bias Measures

<bias_prevention>
- **No Cherry-Picking**: Implement all predefined workloads, report all results
- **Realistic Baselines**: Baseline code must represent what developers actually write
- **Transparent Methodology**: Document every optimization technique used
- **Independent Validation**: Code review baseline implementations for realism
- **Pre-registered Analysis**: All workloads defined before implementation
- **Report Negative Results**: Include workloads where optimization has minimal impact
</bias_prevention>

## Deliverable Requirements

### Code Structure
<repository_structure>
- Follow the exact repository structure defined in PRD section 6.1
- Separate `benches/baseline/` and `benches/optimized/` directories
- Include automated benchmark execution scripts in `scripts/`
- Provide complete environment setup automation
- Use consistent `Cargo.toml` configuration across all benchmarks
</repository_structure>

### Analysis Requirements
<analysis_standards>
- Generate statistical significance tests for all comparisons
- Create visual evidence (charts, graphs, flame graphs)
- Include executive summary with hypothesis test results
- Provide complete reproducibility package with Docker containers
- Export results in JSON format for automated analysis
- Generate HTML reports with criterion.rs
</analysis_standards>

## Success Criteria Enforcement

### Minimum Viable Success
<success_criteria_minimum>
- Demonstrate 2x improvement in any single workload category with statistical significance
- P-value < 0.05 for the improvement
- Effect size Cohen's d > 0.8
</success_criteria_minimum>

### Target Success
<success_criteria_target>
- Achieve ≥2x improvement in 3 out of 5 categories
- All results statistically significant (p < 0.05)
- Large effect sizes (Cohen's d > 0.8) across workloads
- Memory usage reduction of 50-90%
- CPU efficiency improvement of 2-3x
</success_criteria_target>

### Quality Gates
<quality_gates>
- No implementation without corresponding baseline
- No results reporting without statistical validation
- No optimization claims without flamegraph evidence
- No final report without peer-reviewable methodology
- All code must compile and run successfully
- All benchmarks must complete within reasonable time limits
</quality_gates>

## Timeline Adherence

<timeline_enforcement>
- **Day 1**: Complete project setup and infrastructure (Cargo.toml, scripts, data generation)
- **Day 2**: Finish all baseline implementations with initial metrics
- **Day 3**: Complete all optimized implementations with profiling
- **Day 4**: Generate comprehensive benchmark dataset with statistical analysis
- **Day 5**: Deliver complete analysis report with executive summary
</timeline_enforcement>

## Reference Documentation

<documentation_hierarchy>
- Always refer to `/docs/prd.md` as the authoritative specification
- When in doubt about requirements, consult PRD sections 1-8
- For implementation details, reference PRD section 10 code examples
- For success criteria, strictly follow PRD section 8 metrics
- Cross-reference with Rust performance best practices documentation
</documentation_hierarchy>

## Error Prevention

<common_pitfalls>
- Never optimize baseline implementations (defeats the purpose)
- Never use release mode for baseline benchmarks
- Always include warmup runs in benchmarks
- Never compare debug vs release builds directly
- Always use the same dataset for baseline and optimized versions
- Never modify datasets between benchmark runs
- Always validate statistical assumptions before reporting results
</common_pitfalls>
