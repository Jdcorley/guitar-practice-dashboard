// Minimal test to isolate the stack overflow issue
// This tests if basic Slint functionality works

use anyhow::Result;

slint::slint! {
    export component MinimalWindow inherits Window {
        preferred-width: 400px;
        preferred-height: 300px;
        title: "Minimal Test - If you see this, basic Slint works";
        background: #1E1F24;
        
        VerticalLayout {
            alignment: center;
            Text {
                text: "Minimal Slint Test";
                color: white;
                font-size: 20px;
                font-weight: 500;
            }
            Text {
                text: "If you see this window, basic Slint initialization works!";
                color: #9AA0A6;
                font-size: 14px;
            }
        }
    }
}

pub fn run_minimal_test() -> Result<()> {
    eprintln!("[MINIMAL TEST] Starting minimal Slint window...");
    let app = MinimalWindow::new()?;
    eprintln!("[MINIMAL TEST] Window created successfully, running event loop...");
    app.run()?;
    eprintln!("[MINIMAL TEST] Event loop exited normally");
    Ok(())
}

