mod audio;
mod music_theory;

// Minimal test module for diagnostics
#[allow(dead_code)]
mod minimal_test;

use anyhow::Result;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use slint::SharedString;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

use music_theory::{Key, Scale};

slint::include_modules!();

#[derive(Clone, Copy, Debug)]
enum PaneId { TopLeft = 0, TopRight = 1, BottomLeft = 2, BottomRight = 3 }

#[derive(Clone, Debug, Serialize, Deserialize)]
enum ComponentKind { None, Metronome, ChordSheet, VideoPanel, Fretboard, Keys, Scales }

static COMPONENT_NAMES: Lazy<Vec<(ComponentKind, &'static str)>> = Lazy::new(|| {
    vec![
        (ComponentKind::Metronome, "Metronome"),
        (ComponentKind::ChordSheet, "Chord Sheet"),
        (ComponentKind::VideoPanel, "Video Panel"),
        (ComponentKind::Fretboard, "Fretboard"),
        (ComponentKind::Keys, "Keys"),
        (ComponentKind::Scales, "Scales"),
    ]
});

fn apply_component(app: &AppWindow, kind: ComponentKind) {
    let title = match kind {
        ComponentKind::None => "",
        ComponentKind::Metronome => "Metronome",
        ComponentKind::ChordSheet => "Chord Sheet",
        ComponentKind::VideoPanel => "Video Panel",
        ComponentKind::Fretboard => "Fretboard",
        ComponentKind::Keys => "Keys",
        ComponentKind::Scales => "Scales",
    };
    let title = SharedString::from(title);
    let kind_tag = kind_to_tag(kind.clone());

    eprintln!("[apply_component] Setting component: {:?} -> kind_tag: {}, title: {}", kind, kind_tag, title);
    
    app.set_main_title(title);
    app.set_main_kind(kind_tag);
    
    eprintln!("[apply_component] After setting: main_kind = {}", app.get_main_kind());
    
    let _ = save_layout(app);
}

fn kind_to_tag(kind: ComponentKind) -> i32 {
    match kind {
        ComponentKind::None => 0,
        ComponentKind::Metronome => 1,
        ComponentKind::ChordSheet => 2,
        ComponentKind::VideoPanel => 3,
        ComponentKind::Fretboard => 4,
        ComponentKind::Keys => 5,
        ComponentKind::Scales => 6,
    }
}

#[derive(Serialize, Deserialize, Default)]
struct Layout {
    main_kind: i32,
}

fn layout_path() -> std::io::Result<std::path::PathBuf> {
    let base = std::env::var("APPDATA").map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
    let dir = std::path::PathBuf::from(base).join("guitar-practice-dashboard");
    std::fs::create_dir_all(&dir)?;
    Ok(dir.join("layout.json"))
}

fn save_layout(app: &AppWindow) -> std::io::Result<()> {
    let layout = Layout {
        main_kind: app.get_main_kind(),
    };
    let path = layout_path()?;
    let data = serde_json::to_vec_pretty(&layout)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, format!("Failed to serialize layout: {}", e)))?;
    std::fs::write(path, data)
}

fn load_layout(app: &AppWindow) -> std::io::Result<()> {
    let path = layout_path()?;
    if let Ok(bytes) = std::fs::read(path) {
        if let Ok(layout) = serde_json::from_slice::<Layout>(&bytes) {
            app.set_main_kind(layout.main_kind);
            app.set_main_title(title_for(layout.main_kind));
        }
    }
    Ok(())
}

fn title_for(kind_tag: i32) -> SharedString {
    match kind_tag {
        1 => SharedString::from("Metronome"),
        2 => SharedString::from("Chord Sheet"),
        3 => SharedString::from("Video Panel"),
        4 => SharedString::from("Fretboard"),
        5 => SharedString::from("Keys"),
        6 => SharedString::from("Scales"),
        _ => SharedString::from(""),
    }
}

// Generate fret data for a specific string
// Using 12 frets (0-11) to keep component count manageable
// With only 1 pane now, we only create 1 Fretboard = 6 strings × 12 frets = 72 components (safe)
const INITIAL_FRET_COUNT: u8 = 12;

fn generate_string_data(string: i32, key: Key, scale: Scale) -> slint::ModelRc<FretData> {
    let mut data = Vec::with_capacity(INITIAL_FRET_COUNT as usize);
    
    // Generate data for 12 frets (0-11)
    for fret in 0..INITIAL_FRET_COUNT {
        let note = music_theory::get_note_at_position(string as u8, fret);
        let note_name = note.name();
        let is_in_scale = music_theory::is_note_in_scale(note, key, scale);
        
        data.push(FretData {
            string: string,
            fret: fret as i32,
            note_name: SharedString::from(note_name),
            is_in_scale,
        });
    }
    
    slint::ModelRc::new(slint::VecModel::from(data))
}

// Update fret data when key or scale changes
// Use a static flag to prevent infinite recursion
static UPDATING_FRET_DATA: AtomicBool = AtomicBool::new(false);

fn update_fret_data(app: &AppWindow) {
    // Prevent recursive updates - if we're already updating, just return
    if UPDATING_FRET_DATA.compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst).is_err() {
        eprintln!("Warning: Prevented recursive call to update_fret_data");
        return;
    }
    
    // Use defer-like pattern with a guard to ensure flag is reset
    let _guard = FretDataUpdateGuard;
    
    let key = Key::from_int(app.get_selected_key());
    let scale = Scale::from_int(app.get_selected_scale());
    
    // Generate all data first
    let string_0 = generate_string_data(0, key, scale);
    let string_1 = generate_string_data(1, key, scale);
    let string_2 = generate_string_data(2, key, scale);
    let string_3 = generate_string_data(3, key, scale);
    let string_4 = generate_string_data(4, key, scale);
    let string_5 = generate_string_data(5, key, scale);
    
    // Then set all properties at once to minimize property change notifications
    app.set_string_0_data(string_0.into());
    app.set_string_1_data(string_1.into());
    app.set_string_2_data(string_2.into());
    app.set_string_3_data(string_3.into());
    app.set_string_4_data(string_4.into());
    app.set_string_5_data(string_5.into());
}

// Guard to ensure the flag is reset even if we panic
struct FretDataUpdateGuard;

impl Drop for FretDataUpdateGuard {
    fn drop(&mut self) {
        UPDATING_FRET_DATA.store(false, Ordering::SeqCst);
    }
}

fn main() {
    // Capture full backtraces for debugging
    std::env::set_var("RUST_BACKTRACE", "full");
    std::env::set_var("RUST_LIB_BACKTRACE", "1");
    
    // Check for minimal test mode
    if std::env::var("MINIMAL_TEST").is_ok() {
        eprintln!("[MODE] Running minimal test...");
        if let Err(e) = minimal_test::run_minimal_test() {
            eprintln!("[MINIMAL TEST ERROR] {}", e);
            std::process::exit(1);
        }
        return;
    }
    
    // Set up panic hook with enhanced logging
    std::panic::set_hook(Box::new(|panic_info| {
        eprintln!("========================================");
        eprintln!("PANIC DETECTED");
        eprintln!("========================================");
        if let Some(location) = panic_info.location() {
            eprintln!("Location: file '{}' at line {}", location.file(), location.line());
        }
        if let Some(s) = panic_info.payload().downcast_ref::<&str>() {
            eprintln!("Message: {}", s);
        }
        if let Some(s) = panic_info.payload().downcast_ref::<String>() {
            eprintln!("Message: {}", s);
        }
        eprintln!("========================================");
        eprintln!("Full backtrace should be above this");
        eprintln!("========================================");
    }));

    // Feature flags for binary search debugging
    let disable_audio = std::env::var("DISABLE_AUDIO").is_ok();
    let disable_layout = std::env::var("DISABLE_LAYOUT").is_ok();
    let disable_callbacks = std::env::var("DISABLE_CALLBACKS").is_ok();
    
    eprintln!("[CONFIG] Audio: {}", if disable_audio { "DISABLED" } else { "enabled" });
    eprintln!("[CONFIG] Layout: {}", if disable_layout { "DISABLED" } else { "enabled" });
    eprintln!("[CONFIG] Callbacks: {}", if disable_callbacks { "DISABLED" } else { "enabled" });

    // Run the actual main function and handle errors
    if let Err(e) = run_app(disable_audio, disable_layout, disable_callbacks) {
        eprintln!("========================================");
        eprintln!("APPLICATION ERROR");
        eprintln!("========================================");
        eprintln!("Error: {}", e);
        eprintln!("========================================");
        std::process::exit(1);
    }
}

fn run_app(disable_audio: bool, disable_layout: bool, disable_callbacks: bool) -> Result<()> {
    eprintln!("[STEP 1/10] Starting run_app()");
    
    eprintln!("[STEP 2/10] Creating AppWindow...");
    let app = AppWindow::new()?;
    eprintln!("[STEP 2/10] ✓ AppWindow created");

    // Audio initialization (optional)
    eprintln!("[STEP 3/10] Audio initialization...");
    let audio_player: Option<Arc<audio::AudioPlayer>> = if disable_audio {
        eprintln!("[STEP 3/10] ⚠ Audio DISABLED by flag");
        None
    } else {
        match audio::AudioPlayer::new() {
            Ok(player) => {
                eprintln!("[STEP 3/10] ✓ Audio initialized");
                Some(Arc::new(player))
            },
            Err(e) => {
                eprintln!("[STEP 3/10] ⚠ Audio failed: {}", e);
                None
            }
        }
    };

    // Initialize string data - CRITICAL: Start with empty arrays
    // Slint creates components for ALL for-loops during initialization
    // Even empty arrays cause component tree creation, but empty is safer
    eprintln!("[STEP 4/10] Initializing string data arrays as empty...");
    let empty_data: Vec<FretData> = Vec::new();
    let empty_model: slint::ModelRc<FretData> = slint::ModelRc::new(slint::VecModel::from(empty_data));
    app.set_string_0_data(empty_model.clone().into());
    app.set_string_1_data(empty_model.clone().into());
    app.set_string_2_data(empty_model.clone().into());
    app.set_string_3_data(empty_model.clone().into());
    app.set_string_4_data(empty_model.clone().into());
    app.set_string_5_data(empty_model.into());
    eprintln!("[STEP 4/10] ✓ Empty arrays initialized - no FretCells will be created");
    
    // Layout loading (optional)
    if disable_layout {
        eprintln!("[STEP 5/10] ⚠ Layout loading DISABLED by flag");
    } else {
        eprintln!("[STEP 5/10] Loading layout...");
        let _ = load_layout(&app);
        eprintln!("[STEP 5/10] ✓ Layout loaded");
        
        // If layout loaded with Fretboard active, populate fret data
        if app.get_main_kind() == 4 {
            eprintln!("[STEP 5/10] Fretboard detected in layout, populating fret data...");
            update_fret_data(&app);
        }
    }

    // Callbacks setup (optional)
    if disable_callbacks {
        eprintln!("[STEP 6/10] ⚠ Callbacks DISABLED by flag");
    } else {
        eprintln!("[STEP 6/10] Setting up callbacks...");
        
        // Wire up add-component callback - this is called when user clicks a component button in the menu
        {
            let app_weak = app.as_weak();
            app.on_add_component(move |kind| {
                eprintln!("[on_add_component] Received kind: {}", kind);
                if let Some(app) = app_weak.upgrade() {
                    let component_kind = match kind {
                        0 => ComponentKind::None,  // Close button
                        1 => ComponentKind::Metronome,
                        2 => ComponentKind::ChordSheet,
                        3 => ComponentKind::VideoPanel,
                        4 => ComponentKind::Fretboard,
                        5 => ComponentKind::Keys,
                        6 => ComponentKind::Scales,
                        _ => ComponentKind::None,
                    };
                    eprintln!("[on_add_component] Mapped to component_kind: {:?}", component_kind);
                    apply_component(&app, component_kind);
                    // If Fretboard is being shown, ensure fret data is populated
                    if kind == 4 {
                        eprintln!("[on_add_component] Populating fret data for Fretboard...");
                        update_fret_data(&app);
                    }
                } else {
                    eprintln!("[on_add_component] ERROR: Could not upgrade app weak reference");
                }
            });
        }

        // Wire up fretboard interactions
        {
            let audio_player_opt = audio_player.clone();
            app.on_fret_clicked(move |string, fret| {
                let note = music_theory::get_note_at_position(string as u8, fret as u8);
                let frequency = music_theory::calculate_frequency(note);
                if let Some(ref audio_player) = audio_player_opt {
                    audio_player.play_note(frequency);
                }
            });
        }

        // Wire up key selection
        {
            let app_weak = app.as_weak();
            app.on_key_selected(move |key_int| {
                if let Some(app) = app_weak.upgrade() {
                    if app.get_selected_key() != key_int {
                        app.set_selected_key(key_int);
                        update_fret_data(&app);
                    }
                }
            });
        }

        // Wire up scale selection
        {
            let app_weak = app.as_weak();
            app.on_scale_selected(move |scale_int| {
                if let Some(app) = app_weak.upgrade() {
                    if app.get_selected_scale() != scale_int {
                        app.set_selected_scale(scale_int);
                        update_fret_data(&app);
                    }
                }
            });
        }

        
        eprintln!("[STEP 6/10] ✓ Callbacks set up");
    }

    eprintln!("[STEP 7/10] All initialization complete");
    eprintln!("[STEP 8/10] About to call app.run()...");
    eprintln!("[STEP 8/10] ⚠ This is typically where crashes occur!");
    
    let result = app.run();
    
    eprintln!("[STEP 9/10] app.run() returned: {:?}", result);
    
    // Cleanup
    eprintln!("[STEP 10/10] Cleaning up...");
    if let Some(ref audio_player) = audio_player {
        audio_player.cleanup();
    }
    std::thread::sleep(std::time::Duration::from_millis(50));
    
    result?;
    Ok(())
}

fn pane_id_from(i: i32) -> PaneId {
    match i { 0 => PaneId::TopLeft, 1 => PaneId::TopRight, 2 => PaneId::BottomLeft, 3 => PaneId::BottomRight, _ => PaneId::TopLeft }
}
