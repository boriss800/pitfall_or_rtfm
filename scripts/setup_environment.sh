#!/bin/bash
set -euo pipefail

echo "🔧 Setting up benchmark environment according to PRD specifications..."

# Check if running as root for system configuration
if [[ $EUID -eq 0 ]]; then
   echo "❌ This script should not be run as root. Use sudo for individual commands."
   exit 1
fi

# Install required tools
echo "📦 Installing required benchmarking tools..."
if ! command -v cargo &> /dev/null; then
    echo "❌ Rust/Cargo not found. Please install Rust first."
    exit 1
fi

# Install Rust benchmarking tools
cargo install --quiet flamegraph 2>/dev/null || echo "⚠️  flamegraph already installed"
cargo install --quiet hyperfine 2>/dev/null || echo "⚠️  hyperfine already installed"

# Install Python dependencies for analysis
echo "🐍 Setting up Python analysis environment..."
if command -v python3 &> /dev/null; then
    pip3 install --user numpy scipy matplotlib seaborn pandas 2>/dev/null || echo "⚠️  Python packages may already be installed"
else
    echo "⚠️  Python3 not found. Install for statistical analysis."
fi

# System performance configuration (requires sudo)
echo "⚙️  Configuring system for benchmarking (requires sudo)..."

# Set CPU governor to performance mode
echo "🚀 Setting CPU governor to performance mode..."
if command -v cpupower &> /dev/null; then
    sudo cpupower frequency-set --governor performance 2>/dev/null || echo "⚠️  Could not set CPU governor"
else
    echo "⚠️  cpupower not available. Install linux-tools-common for CPU control."
fi

# Disable CPU frequency scaling and turbo boost for consistency
echo "🔒 Disabling turbo boost for consistent results..."
if [[ -f /sys/devices/system/cpu/intel_pstate/no_turbo ]]; then
    echo 1 | sudo tee /sys/devices/system/cpu/intel_pstate/no_turbo > /dev/null 2>&1 || echo "⚠️  Could not disable turbo boost"
fi

# Set all CPU cores to performance governor
for cpu in /sys/devices/system/cpu/cpu*/cpufreq/scaling_governor; do
    if [[ -f "$cpu" ]]; then
        echo performance | sudo tee "$cpu" > /dev/null 2>&1 || true
    fi
done

# Disable ASLR for consistent memory layout
echo "🎯 Disabling ASLR for consistent memory layout..."
echo 0 | sudo tee /proc/sys/kernel/randomize_va_space > /dev/null 2>&1 || echo "⚠️  Could not disable ASLR"

# Create tmpfs for I/O benchmarks (eliminates disk variance)
echo "💾 Setting up tmpfs for I/O benchmarks..."
sudo mkdir -p /tmp/benchmark_data 2>/dev/null || true
if ! mountpoint -q /tmp/benchmark_data; then
    sudo mount -t tmpfs -o size=8G tmpfs /tmp/benchmark_data 2>/dev/null || echo "⚠️  Could not create tmpfs"
fi

# Set up benchmark results directory
echo "📁 Creating results directory structure..."
mkdir -p results/{criterion_reports,flamegraphs,memory_profiles,raw_data}

# Check available memory
echo "💡 System Information:"
echo "  CPU cores: $(nproc)"
echo "  Available memory: $(free -h | awk '/^Mem:/ {print $7}')"
echo "  Tmpfs size: $(df -h /tmp/benchmark_data 2>/dev/null | tail -1 | awk '{print $2}' || echo 'Not mounted')"

# Validate environment
echo "✅ Environment validation:"

# Check CPU governor
current_governor=$(cat /sys/devices/system/cpu/cpu0/cpufreq/scaling_governor 2>/dev/null || echo "unknown")
echo "  CPU governor: $current_governor"

# Check ASLR status
aslr_status=$(cat /proc/sys/kernel/randomize_va_space 2>/dev/null || echo "unknown")
echo "  ASLR disabled: $([ "$aslr_status" = "0" ] && echo "✅" || echo "❌")"

# Check tmpfs
tmpfs_status=$(mountpoint -q /tmp/benchmark_data && echo "✅" || echo "❌")
echo "  Tmpfs mounted: $tmpfs_status"

# Check required tools
echo "  flamegraph: $(command -v flamegraph >/dev/null && echo "✅" || echo "❌")"
echo "  hyperfine: $(command -v hyperfine >/dev/null && echo "✅" || echo "❌")"

echo ""
echo "🎯 Environment setup complete!"
echo "📋 Next steps:"
echo "  1. Run 'cargo run --bin generate_data' to create benchmark datasets"
echo "  2. Run './scripts/run_benchmarks.sh' to execute the benchmark suite"
echo ""
echo "⚠️  Note: Some system configurations require sudo and may need manual setup"
echo "    if the script couldn't apply them automatically."

# Create a restore script for cleanup
cat > scripts/restore_environment.sh << 'EOF'
#!/bin/bash
echo "🔄 Restoring system environment..."

# Restore CPU governor to ondemand/powersave
sudo cpupower frequency-set --governor ondemand 2>/dev/null || echo "Could not restore CPU governor"

# Re-enable turbo boost
if [[ -f /sys/devices/system/cpu/intel_pstate/no_turbo ]]; then
    echo 0 | sudo tee /sys/devices/system/cpu/intel_pstate/no_turbo > /dev/null 2>&1 || true
fi

# Re-enable ASLR
echo 2 | sudo tee /proc/sys/kernel/randomize_va_space > /dev/null 2>&1 || true

# Unmount tmpfs
sudo umount /tmp/benchmark_data 2>/dev/null || true
sudo rmdir /tmp/benchmark_data 2>/dev/null || true

echo "✅ Environment restored to default settings"
EOF

chmod +x scripts/restore_environment.sh
echo "💾 Created scripts/restore_environment.sh for cleanup"
