# Diagnostic Testing Guide

This guide will help you systematically identify where the stack overflow is occurring.

## Quick Start

### Test 1: Minimal Slint Window
Tests if basic Slint works on your system:
```powershell
$env:MINIMAL_TEST = "1"
cargo run
```
**Expected:** A small window appears with "Minimal Slint Test" text
**If it fails:** Slint setup or Windows issue
**If it passes:** The problem is in our application code

### Test 2: Full App Without Audio
```powershell
Remove-Item Env:\MINIMAL_TEST
$env:DISABLE_AUDIO = "1"
$env:RUST_BACKTRACE = "full"
cargo run 2>&1 | Tee-Object -FilePath "test2.log"
```
**Check the output:** Look for the last `[STEP X/10]` message to see where it crashes

### Test 3: Without Layout Loading
```powershell
$env:DISABLE_AUDIO = "1"
$env:DISABLE_LAYOUT = "1"
$env:RUST_BACKTRACE = "full"
cargo run 2>&1 | Tee-Object -FilePath "test3.log"
```

### Test 4: Without Callbacks
```powershell
$env:DISABLE_AUDIO = "1"
$env:DISABLE_LAYOUT = "1"
$env:DISABLE_CALLBACKS = "1"
$env:RUST_BACKTRACE = "full"
cargo run 2>&1 | Tee-Object -FilePath "test4.log"
```

## Automated Testing

Run all tests automatically:
```powershell
.\test.ps1
```

This will run all tests in sequence and save output to log files.

## Understanding the Output

Look for messages like:
- `[STEP 1/10]` through `[STEP 10/10]` - Shows initialization progress
- `✓` - Step completed successfully
- `⚠` - Step was disabled or had warnings
- The last `[STEP X/10]` message shows where the crash occurred

## Reporting Results

When reporting issues, please include:
1. Which test passed/failed
2. The last `[STEP X/10]` message you saw
3. Any panic messages or stack traces
4. The contents of the generated log file (if any)

## Manual Testing

You can also combine flags:
```powershell
# Test with only minimal features
$env:DISABLE_AUDIO = "1"
$env:DISABLE_LAYOUT = "1"
$env:DISABLE_CALLBACKS = "1"
$env:RUST_BACKTRACE = "full"
cargo run
```

This helps isolate which specific component causes the crash.

