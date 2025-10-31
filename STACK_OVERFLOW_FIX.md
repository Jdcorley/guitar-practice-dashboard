# Stack Overflow Issue - Resolved

## Problem
The application was crashing with `STATUS_STACK_OVERFLOW` immediately during `AppWindow::new()` initialization, before any Rust code could execute.

## Root Cause
Slint creates **all component instances** during window initialization, even when components have `visible: false`. 

With 4 Panes, each containing a Fretboard component with 6 `for` loops (each iterating over up to 25 items), Slint was trying to create:
- 4 Pane components
- 4 Fretboard components (one per pane, even though invisible)
- Each Fretboard: 6 `for` loops × 25 potential items = 150+ component instances
- **Total: ~600+ component instances during initialization**

This caused the stack to overflow during Slint's initialization phase.

## Solution
**Currently:** Fretboard component is disabled with a placeholder. Even with reduced frets (12 instead of 25), creating 4 Fretboard instances still causes stack overflow.

**Root cause:** Slint creates ALL component instances during `AppWindow::new()` initialization, even when `visible: false`. There is no lazy loading or conditional component creation in Slint 1.5.

**Temporary fix:** Replaced Fretboard with a simple Rectangle placeholder.

**Future solutions needed:**
1. Use Canvas-based rendering instead of component-based (render fretboard as pixels, not components)
2. Implement single Fretboard instance that can be shared/reused
3. Wait for Slint to support lazy component creation
4. Reduce to a much smaller number of frets (tested: 12 still too many with 4 panes)

## Diagnostic Process Used
1. Created minimal test - confirmed basic Slint works
2. Added step-by-step diagnostic logging (`[STEP X/10]`)
3. Used feature flags (`DISABLE_AUDIO`, `DISABLE_LAYOUT`, `DISABLE_CALLBACKS`)
4. Identified crash occurs at `AppWindow::new()` - before Rust code runs
5. Confirmed issue is in Slint UI definition, not Rust code
6. Identified Fretboard component as the problematic component

## Files Modified
- `ui/main.slint` - Temporarily disabled Fretboard component
- `src/main.rs` - Added comprehensive diagnostic logging
- `src/minimal_test.rs` - Created minimal test to isolate issues
- `test.ps1` - Automated diagnostic testing script

## Future Work
Need to properly fix the Fretboard component by:
1. Using lazy rendering/deferred component creation
2. Limiting initial component count
3. Using virtualized lists if available in Slint
4. Restructuring component hierarchy to avoid deep nesting

## Key Learnings
- Slint creates all components during initialization, not lazily
- `visible: false` hides but doesn't prevent component creation
- Large `for` loops in Slint can cause stack overflow
- Diagnostic logging with step markers is essential for debugging

