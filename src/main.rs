use anyhow::Result;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use slint::SharedString;

slint::include_modules!();

#[derive(Clone, Copy, Debug)]
enum PaneId { TopLeft = 0, TopRight = 1, BottomLeft = 2, BottomRight = 3 }

#[derive(Clone, Debug, Serialize, Deserialize)]
enum ComponentKind { None, Metronome, ChordSheet, VideoPanel }

static COMPONENT_NAMES: Lazy<Vec<(ComponentKind, &'static str)>> = Lazy::new(|| {
    vec![
        (ComponentKind::Metronome, "Metronome"),
        (ComponentKind::ChordSheet, "Chord Sheet"),
        (ComponentKind::VideoPanel, "Video Panel"),
    ]
});

fn apply_component(app: &AppWindow, pane: PaneId, kind: ComponentKind) {
    let title = match kind {
        ComponentKind::None => "",
        ComponentKind::Metronome => "Metronome",
        ComponentKind::ChordSheet => "Chord Sheet",
        ComponentKind::VideoPanel => "Video Panel",
    };
    let title = SharedString::from(title);

    match pane {
        PaneId::TopLeft => { app.set_tl_title(title.clone()); app.set_tl_kind(kind_to_tag(kind)); }
        PaneId::TopRight => { app.set_tr_title(title.clone()); app.set_tr_kind(kind_to_tag(kind)); }
        PaneId::BottomLeft => { app.set_bl_title(title.clone()); app.set_bl_kind(kind_to_tag(kind)); }
        PaneId::BottomRight => { app.set_br_title(title.clone()); app.set_br_kind(kind_to_tag(kind)); }
    }
    let _ = save_layout(app);
}

fn kind_to_tag(kind: ComponentKind) -> i32 {
    match kind {
        ComponentKind::None => 0,
        ComponentKind::Metronome => 1,
        ComponentKind::ChordSheet => 2,
        ComponentKind::VideoPanel => 3,
    }
}

#[derive(Serialize, Deserialize, Default)]
struct Layout {
    tl_kind: i32,
    tr_kind: i32,
    bl_kind: i32,
    br_kind: i32,
}

fn layout_path() -> std::io::Result<std::path::PathBuf> {
    let base = std::env::var("APPDATA").map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
    let dir = std::path::PathBuf::from(base).join("guitar-practice-dashboard");
    std::fs::create_dir_all(&dir)?;
    Ok(dir.join("layout.json"))
}

fn save_layout(app: &AppWindow) -> std::io::Result<()> {
    let layout = Layout {
        tl_kind: app.get_tl_kind(),
        tr_kind: app.get_tr_kind(),
        bl_kind: app.get_bl_kind(),
        br_kind: app.get_br_kind(),
    };
    let path = layout_path()?;
    let data = serde_json::to_vec_pretty(&layout).unwrap();
    std::fs::write(path, data)
}

fn load_layout(app: &AppWindow) -> std::io::Result<()> {
    let path = layout_path()?;
    if let Ok(bytes) = std::fs::read(path) {
        if let Ok(layout) = serde_json::from_slice::<Layout>(&bytes) {
            app.set_tl_kind(layout.tl_kind);
            app.set_tr_kind(layout.tr_kind);
            app.set_bl_kind(layout.bl_kind);
            app.set_br_kind(layout.br_kind);
            app.set_tl_title(title_for(layout.tl_kind));
            app.set_tr_title(title_for(layout.tr_kind));
            app.set_bl_title(title_for(layout.bl_kind));
            app.set_br_title(title_for(layout.br_kind));
        }
    }
    Ok(())
}

fn title_for(kind_tag: i32) -> SharedString {
    match kind_tag {
        1 => SharedString::from("Metronome"),
        2 => SharedString::from("Chord Sheet"),
        3 => SharedString::from("Video Panel"),
        _ => SharedString::from(""),
    }
}

fn main() -> Result<()> {
    let app = AppWindow::new()?;

    // Restore persisted layout if available
    let _ = load_layout(&app);

    { let app_weak = app.as_weak();
      app.on_open_metronome(move |pane| { if let Some(app) = app_weak.upgrade() {
          apply_component(&app, pane_id_from(pane), ComponentKind::Metronome);
      }}); }
    { let app_weak = app.as_weak();
      app.on_open_chord_sheet(move |pane| { if let Some(app) = app_weak.upgrade() {
          apply_component(&app, pane_id_from(pane), ComponentKind::ChordSheet);
      }}); }
    { let app_weak = app.as_weak();
      app.on_open_video_panel(move |pane| { if let Some(app) = app_weak.upgrade() {
          apply_component(&app, pane_id_from(pane), ComponentKind::VideoPanel);
      }}); }
    { let app_weak = app.as_weak();
      app.on_close_pane(move |pane| { if let Some(app) = app_weak.upgrade() {
          apply_component(&app, pane_id_from(pane), ComponentKind::None);
      }}); }

    app.run()?;
    Ok(())
}

fn pane_id_from(i: i32) -> PaneId {
    match i { 0 => PaneId::TopLeft, 1 => PaneId::TopRight, 2 => PaneId::BottomLeft, 3 => PaneId::BottomRight, _ => PaneId::TopLeft }
}
