use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use ahash::AHashMap;

/// Validation framework to ensure baseline and optimized implementations produce identical results
pub struct BenchmarkValidator {
    pub tolerance: f64,
    pub max_string_diff: usize,
}

impl Default for BenchmarkValidator {
    fn default() -> Self {
        Self {
            tolerance: 1e-10, // Very strict tolerance for floating point comparisons
            max_string_diff: 0, // No differences allowed in string outputs
        }
    }
}

impl BenchmarkValidator {
    pub fn new(tolerance: f64) -> Self {
        Self {
            tolerance,
            max_string_diff: 0,
        }
    }

    /// Compare two floating point values with tolerance
    pub fn compare_f64(&self, baseline: f64, optimized: f64) -> bool {
        if baseline.is_nan() && optimized.is_nan() {
            return true;
        }
        if baseline.is_infinite() && optimized.is_infinite() {
            return baseline.signum() == optimized.signum();
        }
        (baseline - optimized).abs() <= self.tolerance
    }

    /// Compare two vectors of floating point values
    pub fn compare_f64_vec(&self, baseline: &[f64], optimized: &[f64]) -> ValidationResult {
        if baseline.len() != optimized.len() {
            return ValidationResult::failure(format!(
                "Length mismatch: baseline={}, optimized={}",
                baseline.len(),
                optimized.len()
            ));
        }

        for (i, (&b, &o)) in baseline.iter().zip(optimized.iter()).enumerate() {
            if !self.compare_f64(b, o) {
                return ValidationResult::failure(format!(
                    "Value mismatch at index {}: baseline={}, optimized={}, diff={}",
                    i, b, o, (b - o).abs()
                ));
            }
        }

        ValidationResult::success()
    }

    /// Compare two integer vectors
    pub fn compare_int_vec<T: PartialEq + std::fmt::Debug>(&self, baseline: &[T], optimized: &[T]) -> ValidationResult {
        if baseline.len() != optimized.len() {
            return ValidationResult::failure(format!(
                "Length mismatch: baseline={}, optimized={}",
                baseline.len(),
                optimized.len()
            ));
        }

        for (i, (b, o)) in baseline.iter().zip(optimized.iter()).enumerate() {
            if b != o {
                return ValidationResult::failure(format!(
                    "Value mismatch at index {}: baseline={:?}, optimized={:?}",
                    i, b, o
                ));
            }
        }

        ValidationResult::success()
    }

    /// Compare two strings
    pub fn compare_strings(&self, baseline: &str, optimized: &str) -> ValidationResult {
        if baseline == optimized {
            ValidationResult::success()
        } else {
            ValidationResult::failure(format!(
                "String mismatch: baseline length={}, optimized length={}",
                baseline.len(),
                optimized.len()
            ))
        }
    }

    /// Compare two hash maps
    pub fn compare_hashmaps<K, V>(&self, baseline: &HashMap<K, V>, optimized: &AHashMap<K, V>) -> ValidationResult 
    where
        K: std::hash::Hash + Eq + std::fmt::Debug,
        V: PartialEq + std::fmt::Debug,
    {
        if baseline.len() != optimized.len() {
            return ValidationResult::failure(format!(
                "HashMap size mismatch: baseline={}, optimized={}",
                baseline.len(),
                optimized.len()
            ));
        }

        for (key, baseline_value) in baseline {
            match optimized.get(key) {
                Some(optimized_value) => {
                    if baseline_value != optimized_value {
                        return ValidationResult::failure(format!(
                            "Value mismatch for key {:?}: baseline={:?}, optimized={:?}",
                            key, baseline_value, optimized_value
                        ));
                    }
                }
                None => {
                    return ValidationResult::failure(format!(
                        "Key {:?} missing in optimized hashmap",
                        key
                    ));
                }
            }
        }

        ValidationResult::success()
    }

    /// Compare file contents
    pub fn compare_files(&self, baseline_path: &str, optimized_path: &str) -> ValidationResult {
        let baseline_content = match std::fs::read_to_string(baseline_path) {
            Ok(content) => content,
            Err(e) => return ValidationResult::failure(format!("Failed to read baseline file: {}", e)),
        };

        let optimized_content = match std::fs::read_to_string(optimized_path) {
            Ok(content) => content,
            Err(e) => return ValidationResult::failure(format!("Failed to read optimized file: {}", e)),
        };

        self.compare_strings(&baseline_content, &optimized_content)
    }

    /// Validate that two numeric results are approximately equal
    pub fn validate_numeric_result(&self, baseline: f64, optimized: f64, test_name: &str) -> ValidationResult {
        if self.compare_f64(baseline, optimized) {
            ValidationResult::success()
        } else {
            ValidationResult::failure(format!(
                "{}: Numeric mismatch - baseline={}, optimized={}, diff={}",
                test_name,
                baseline,
                optimized,
                (baseline - optimized).abs()
            ))
        }
    }
}

#[derive(Debug, Clone)]
pub struct ValidationResult {
    pub passed: bool,
    pub message: String,
}

impl ValidationResult {
    pub fn success() -> Self {
        Self {
            passed: true,
            message: "Validation passed".to_string(),
        }
    }

    pub fn failure(message: String) -> Self {
        Self {
            passed: false,
            message,
        }
    }

    pub fn combine(results: Vec<ValidationResult>) -> Self {
        let failed_results: Vec<_> = results.iter().filter(|r| !r.passed).collect();
        
        if failed_results.is_empty() {
            Self::success()
        } else {
            let combined_message = failed_results
                .iter()
                .map(|r| r.message.as_str())
                .collect::<Vec<_>>()
                .join("; ");
            
            Self::failure(combined_message)
        }
    }
}

/// Trait for benchmarks that can be validated
pub trait ValidatableBenchmark {
    type Output;
    
    fn run_baseline(&self) -> io::Result<Self::Output>;
    fn run_optimized(&self) -> io::Result<Self::Output>;
    fn validate_outputs(&self, baseline: &Self::Output, optimized: &Self::Output) -> ValidationResult;
    
    fn validate(&self) -> ValidationResult {
        let baseline_result = match self.run_baseline() {
            Ok(result) => result,
            Err(e) => return ValidationResult::failure(format!("Baseline failed: {}", e)),
        };

        let optimized_result = match self.run_optimized() {
            Ok(result) => result,
            Err(e) => return ValidationResult::failure(format!("Optimized failed: {}", e)),
        };

        self.validate_outputs(&baseline_result, &optimized_result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_f64_comparison() {
        let validator = BenchmarkValidator::default();
        
        assert!(validator.compare_f64(1.0, 1.0));
        assert!(validator.compare_f64(1.0, 1.0 + 1e-12));
        assert!(!validator.compare_f64(1.0, 1.1));
        
        // Test special values
        assert!(validator.compare_f64(f64::NAN, f64::NAN));
        assert!(validator.compare_f64(f64::INFINITY, f64::INFINITY));
        assert!(validator.compare_f64(f64::NEG_INFINITY, f64::NEG_INFINITY));
        assert!(!validator.compare_f64(f64::INFINITY, f64::NEG_INFINITY));
    }

    #[test]
    fn test_vector_comparison() {
        let validator = BenchmarkValidator::default();
        
        let baseline = vec![1, 2, 3, 4, 5];
        let optimized = vec![1, 2, 3, 4, 5];
        let result = validator.compare_int_vec(&baseline, &optimized);
        assert!(result.passed);
        
        let optimized_wrong = vec![1, 2, 3, 4, 6];
        let result = validator.compare_int_vec(&baseline, &optimized_wrong);
        assert!(!result.passed);
    }
}
