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
