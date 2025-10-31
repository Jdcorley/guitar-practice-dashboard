# Fretboard Component Stack Overflow - Solution Options

## Problem Summary
The Fretboard component causes stack overflow because:
- Slint creates **ALL component instances** during `AppWindow::new()` initialization
- Each Pane tries to create a Fretboard instance (4 panes = 4 instances)
- Each Fretboard has 6 `for` loops with 12+ items = 72+ FretCell components per instance
- Total: 4 × 72 = 288+ components just for Fretboards during initialization
- This exceeds the Windows default stack size (1MB typically)

## Current Status
✅ **Fretboard is DISABLED** - Using placeholder Rectangle to prevent crashes

## Solution Options

### Option 1: Canvas-Based Rendering (RECOMMENDED)
Instead of using components for each fret cell, draw the fretboard using Slint's Canvas API:
- Render fretboard as pixels/graphics, not component tree
- Single Canvas component instead of 150+ FretCell components
- Much lighter on stack memory

**Implementation:** Would need to create a custom rendering function in Rust that draws the fretboard.

### Option 2: Single Shared Fretboard Instance
Create only ONE Fretboard at AppWindow level, position it dynamically:
- Create Fretboard once at AppWindow level
- Use absolute positioning or layout management to show it in the active pane
- Only one instance instead of 4

**Limitation:** Only one Fretboard visible at a time, users can't have multiple fretboards.

### Option 3: Drastically Reduce Component Count
- Reduce to 6 frets (0-5) instead of 12 or 25
- This would be: 6 strings × 6 frets = 36 components per Fretboard
- 4 panes = 144 components total (might still be too many)

### Option 4: Wait for Slint Lazy Loading
- Slint may add lazy component creation in future versions
- Component instances would only be created when actually visible
- Current version (1.5) doesn't support this

### Option 5: Restructure UI
- Allow only ONE Fretboard pane active at a time
- Disable ability to have Fretboard in multiple panes simultaneously
- This way only one Fretboard needs to be created

## Recommended Approach
**Option 1 (Canvas-based)** is the best long-term solution:
- Most flexible
- Scalable to any number of frets
- Better performance
- Single component regardless of data size

## Implementation Notes
To implement Canvas rendering:
1. Use `slint::Canvas` or similar API (check Slint 1.5 docs)
2. Create Rust function to render fretboard to canvas
3. Replace Fretboard component with Canvas component
4. Handle clicks by calculating fret position from mouse coordinates

