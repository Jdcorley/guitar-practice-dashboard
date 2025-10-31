Write-Host "=== Diagnostic Test Suite ===" -ForegroundColor Cyan

# Test 1: Minimal Slint window
Write-Host ""
Write-Host "[Test 1] Minimal Slint window..." -ForegroundColor Yellow
$env:MINIMAL_TEST = "1"
cargo run 2>&1 | Select-Object -Last 10
if ($LASTEXITCODE -eq 0) {
    Write-Host "[OK] Test 1 PASSED" -ForegroundColor Green
} else {
    Write-Host "[FAIL] Test 1 FAILED" -ForegroundColor Red
    Remove-Item Env:\MINIMAL_TEST -ErrorAction SilentlyContinue
    exit 1
}

Remove-Item Env:\MINIMAL_TEST -ErrorAction SilentlyContinue

# Test 2: Full app without audio
Write-Host ""
Write-Host "[Test 2] Full app without audio..." -ForegroundColor Yellow
$env:DISABLE_AUDIO = "1"
$env:RUST_BACKTRACE = "full"
cargo run 2>&1 | Tee-Object -FilePath "test2_output.log" | Select-Object -Last 20
Remove-Item Env:\DISABLE_AUDIO -ErrorAction SilentlyContinue
Write-Host ""
Write-Host "Full output saved to test2_output.log" -ForegroundColor Cyan
Write-Host "Check the last [STEP X/10] message above to see where it crashed" -ForegroundColor Yellow

# Test 3: Without layout loading
Write-Host ""
Write-Host "[Test 3] Without layout loading..." -ForegroundColor Yellow
$env:DISABLE_AUDIO = "1"
$env:DISABLE_LAYOUT = "1"
$env:RUST_BACKTRACE = "full"
cargo run 2>&1 | Tee-Object -FilePath "test3_output.log" | Select-Object -Last 20
Remove-Item Env:\DISABLE_AUDIO -ErrorAction SilentlyContinue
Remove-Item Env:\DISABLE_LAYOUT -ErrorAction SilentlyContinue
Write-Host ""
Write-Host "Full output saved to test3_output.log" -ForegroundColor Cyan

# Test 4: Without callbacks
Write-Host ""
Write-Host "[Test 4] Without callbacks..." -ForegroundColor Yellow
$env:DISABLE_AUDIO = "1"
$env:DISABLE_LAYOUT = "1"
$env:DISABLE_CALLBACKS = "1"
$env:RUST_BACKTRACE = "full"
cargo run 2>&1 | Tee-Object -FilePath "test4_output.log" | Select-Object -Last 20
Remove-Item Env:\DISABLE_AUDIO -ErrorAction SilentlyContinue
Remove-Item Env:\DISABLE_LAYOUT -ErrorAction SilentlyContinue
Remove-Item Env:\DISABLE_CALLBACKS -ErrorAction SilentlyContinue
Write-Host ""
Write-Host "Full output saved to test4_output.log" -ForegroundColor Cyan

Write-Host ""
Write-Host "=== Tests Complete ===" -ForegroundColor Cyan
Write-Host "Check the log files to see where each test failed" -ForegroundColor Yellow
