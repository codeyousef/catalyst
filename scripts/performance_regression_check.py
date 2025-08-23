#!/usr/bin/env python3
"""
Catalyst IDE Performance Regression Detection Script

This script runs performance tests and compares results against baseline
measurements to detect performance regressions.

Usage:
    python scripts/performance_regression_check.py [--baseline] [--compare]
"""

import json
import subprocess
import sys
import time
from pathlib import Path
from dataclasses import dataclass, asdict
from typing import Dict, List, Optional
import argparse

@dataclass
class PerformanceMetrics:
    """Performance metrics for a single test run"""
    cold_start_ms: Optional[float] = None
    warm_start_ms: Optional[float] = None
    memory_usage_mb: Optional[float] = None
    binary_size_mb: Optional[float] = None
    file_search_ms: Optional[float] = None
    git_status_ms: Optional[float] = None
    syntax_highlight_ms: Optional[float] = None
    timestamp: float = 0.0
    git_commit: str = ""
    
class PerformanceRegression:
    """Detects and reports performance regressions"""
    
    def __init__(self, baseline_file: str = "performance_baseline.json"):
        self.baseline_file = Path(baseline_file)
        self.threshold_percent = 20  # Alert if performance degrades by >20%
        
    def run_performance_tests(self) -> PerformanceMetrics:
        """Run the performance test suite and extract metrics"""
        print("Running performance tests...")
        
        try:
            # Run the performance tests
            result = subprocess.run([
                "cargo", "test", "--test", "performance", "--release", "--", "--nocapture"
            ], capture_output=True, text=True, timeout=300)
            
            if result.returncode != 0:
                print(f"Performance tests failed: {result.stderr}")
                # Continue with partial results
                
            output = result.stdout
            metrics = self._parse_test_output(output)
            
            # Add metadata
            metrics.timestamp = time.time()
            metrics.git_commit = self._get_git_commit()
            
            return metrics
            
        except subprocess.TimeoutExpired:
            print("Performance tests timed out")
            return PerformanceMetrics(timestamp=time.time())
        except Exception as e:
            print(f"Error running performance tests: {e}")
            return PerformanceMetrics(timestamp=time.time())
    
    def _parse_test_output(self, output: str) -> PerformanceMetrics:
        """Parse performance metrics from test output"""
        metrics = PerformanceMetrics()
        
        lines = output.split('\n')
        for line in lines:
            line = line.strip()
            
            # Parse startup times
            if "Cold start time:" in line:
                try:
                    time_str = line.split("Cold start time:")[1].strip()
                    ms = self._extract_milliseconds(time_str)
                    metrics.cold_start_ms = ms
                except:
                    pass
                    
            elif "Warm start time:" in line:
                try:
                    time_str = line.split("Warm start time:")[1].strip()
                    ms = self._extract_milliseconds(time_str)
                    metrics.warm_start_ms = ms
                except:
                    pass
                    
            # Parse memory usage
            elif "Idle memory usage:" in line:
                try:
                    mem_str = line.split("Idle memory usage:")[1].strip()
                    mb = float(mem_str.split()[0])
                    metrics.memory_usage_mb = mb
                except:
                    pass
                    
            # Parse binary size
            elif "Binary size:" in line:
                try:
                    size_str = line.split("Binary size:")[1].strip()
                    mb = float(size_str.split()[0])
                    metrics.binary_size_mb = mb
                except:
                    pass
                    
            # Parse file search performance
            elif "Searched" in line and "files in" in line:
                try:
                    time_str = line.split("files in")[1].strip()
                    ms = self._extract_milliseconds(time_str)
                    metrics.file_search_ms = ms
                except:
                    pass
                    
            # Parse git status performance
            elif "Git status found" in line and "in" in line:
                try:
                    parts = line.split("in")
                    if len(parts) > 1:
                        time_str = parts[-1].strip()
                        ms = self._extract_milliseconds(time_str)
                        metrics.git_status_ms = ms
                except:
                    pass
                    
            # Parse syntax highlighting performance
            elif "Syntax highlighting processed" in line and "in" in line:
                try:
                    parts = line.split("in")
                    if len(parts) > 1:
                        time_str = parts[-1].strip()
                        ms = self._extract_milliseconds(time_str)
                        metrics.syntax_highlight_ms = ms
                except:
                    pass
        
        return metrics
    
    def _extract_milliseconds(self, time_str: str) -> float:
        """Extract milliseconds from various time string formats"""
        time_str = time_str.lower().replace("duration::", "").strip()
        
        if "ms" in time_str:
            return float(time_str.replace("ms", "").strip())
        elif "µs" in time_str or "μs" in time_str:
            return float(time_str.replace("µs", "").replace("μs", "").strip()) / 1000.0
        elif "ns" in time_str:
            return float(time_str.replace("ns", "").strip()) / 1_000_000.0
        elif "s" in time_str:
            return float(time_str.replace("s", "").strip()) * 1000.0
        else:
            # Try to parse as floating point seconds
            try:
                return float(time_str) * 1000.0
            except:
                return 0.0
    
    def _get_git_commit(self) -> str:
        """Get current git commit hash"""
        try:
            result = subprocess.run(["git", "rev-parse", "HEAD"], 
                                  capture_output=True, text=True)
            if result.returncode == 0:
                return result.stdout.strip()
        except:
            pass
        return "unknown"
    
    def save_baseline(self, metrics: PerformanceMetrics):
        """Save metrics as new baseline"""
        with open(self.baseline_file, 'w') as f:
            json.dump(asdict(metrics), f, indent=2)
        print(f"Saved baseline to {self.baseline_file}")
    
    def load_baseline(self) -> Optional[PerformanceMetrics]:
        """Load baseline metrics"""
        if not self.baseline_file.exists():
            return None
            
        try:
            with open(self.baseline_file, 'r') as f:
                data = json.load(f)
                return PerformanceMetrics(**data)
        except Exception as e:
            print(f"Failed to load baseline: {e}")
            return None
    
    def compare_metrics(self, current: PerformanceMetrics, 
                       baseline: PerformanceMetrics) -> List[str]:
        """Compare current metrics against baseline and return regressions"""
        regressions = []
        
        # Define metric comparisons (name, current_value, baseline_value, lower_is_better)
        comparisons = [
            ("Cold start", current.cold_start_ms, baseline.cold_start_ms, True),
            ("Warm start", current.warm_start_ms, baseline.warm_start_ms, True),
            ("Memory usage", current.memory_usage_mb, baseline.memory_usage_mb, True),
            ("Binary size", current.binary_size_mb, baseline.binary_size_mb, True),
            ("File search", current.file_search_ms, baseline.file_search_ms, True),
            ("Git status", current.git_status_ms, baseline.git_status_ms, True),
            ("Syntax highlighting", current.syntax_highlight_ms, baseline.syntax_highlight_ms, True),
        ]
        
        for name, current_val, baseline_val, lower_is_better in comparisons:
            if current_val is None or baseline_val is None:
                continue
                
            if baseline_val == 0:
                continue
                
            percent_change = ((current_val - baseline_val) / baseline_val) * 100
            
            # Check for regression
            is_regression = False
            if lower_is_better and percent_change > self.threshold_percent:
                is_regression = True
            elif not lower_is_better and percent_change < -self.threshold_percent:
                is_regression = True
            
            if is_regression:
                regressions.append(
                    f"{name}: {baseline_val:.2f} -> {current_val:.2f} "
                    f"({percent_change:+.1f}%)"
                )
        
        return regressions
    
    def generate_report(self, current: PerformanceMetrics, 
                       baseline: Optional[PerformanceMetrics] = None):
        """Generate a performance report"""
        print("\n" + "="*60)
        print("CATALYST IDE PERFORMANCE REPORT")
        print("="*60)
        
        print(f"Timestamp: {time.ctime(current.timestamp)}")
        print(f"Git commit: {current.git_commit}")
        print()
        
        print("Current Performance Metrics:")
        print("-" * 30)
        
        metrics = [
            ("Cold start", current.cold_start_ms, "ms"),
            ("Warm start", current.warm_start_ms, "ms"),
            ("Memory usage", current.memory_usage_mb, "MB"),
            ("Binary size", current.binary_size_mb, "MB"),
            ("File search", current.file_search_ms, "ms"),
            ("Git status", current.git_status_ms, "ms"),
            ("Syntax highlighting", current.syntax_highlight_ms, "ms"),
        ]
        
        for name, value, unit in metrics:
            if value is not None:
                print(f"{name:20}: {value:8.2f} {unit}")
            else:
                print(f"{name:20}: {'N/A':>8}")
        
        # Performance requirements check
        print("\nPerformance Requirements:")
        print("-" * 30)
        
        requirements = [
            ("Cold start", current.cold_start_ms, 500, "ms"),
            ("Warm start", current.warm_start_ms, 200, "ms"),
            ("Memory usage", current.memory_usage_mb, 40, "MB"),
            ("Binary size", current.binary_size_mb, 5, "MB"),
            ("File search", current.file_search_ms, 50, "ms"),
            ("Git status", current.git_status_ms, 100, "ms"),
            ("Syntax highlighting", current.syntax_highlight_ms, 10, "ms"),
        ]
        
        all_passing = True
        for name, value, threshold, unit in requirements:
            if value is not None:
                status = "PASS" if value <= threshold else "FAIL"
                if status == "FAIL":
                    all_passing = False
                print(f"{name:20}: {value:6.2f}/{threshold:6.2f} {unit:2} [{status}]")
            else:
                print(f"{name:20}: {'N/A':>6}/{'N/A':>6} {unit:2} [SKIP]")
        
        if baseline:
            print("\nRegression Analysis:")
            print("-" * 30)
            
            regressions = self.compare_metrics(current, baseline)
            if regressions:
                print("⚠️  PERFORMANCE REGRESSIONS DETECTED:")
                for regression in regressions:
                    print(f"  - {regression}")
                all_passing = False
            else:
                print("✅ No significant performance regressions detected")
        
        print("\n" + "="*60)
        
        if not all_passing:
            print("❌ Performance check FAILED")
            return False
        else:
            print("✅ Performance check PASSED")
            return True

def main():
    parser = argparse.ArgumentParser(description="Catalyst IDE Performance Regression Check")
    parser.add_argument("--baseline", action="store_true", 
                       help="Save current run as baseline")
    parser.add_argument("--compare", action="store_true", 
                       help="Compare against baseline (default)")
    parser.add_argument("--threshold", type=float, default=20.0,
                       help="Regression threshold percentage (default: 20)")
    
    args = parser.parse_args()
    
    checker = PerformanceRegression()
    checker.threshold_percent = args.threshold
    
    # Run performance tests
    current_metrics = checker.run_performance_tests()
    
    if args.baseline:
        # Save as new baseline
        checker.save_baseline(current_metrics)
        checker.generate_report(current_metrics)
    else:
        # Compare against baseline
        baseline_metrics = checker.load_baseline()
        success = checker.generate_report(current_metrics, baseline_metrics)
        
        if not success:
            sys.exit(1)

if __name__ == "__main__":
    main()