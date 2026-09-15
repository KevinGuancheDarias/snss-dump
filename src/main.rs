use snss::{SessionStore, SourceKind};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

/// Resolve the concrete Sessions directory from a user-supplied Chrome dir.
///
/// Accepts, in order:
///   .../User Data/Default/Sessions            (direct)
///   .../User Data/Default                     (profile dir)
///   .../User Data                             (profile parent, single profile)
fn resolve_sessions_dir(input: &str) -> Result<PathBuf, String> {
    let p = Path::new(input);
    let is_sessions_dir = |d: &Path| {
        d.is_dir() && fs::read_dir(d).map(|e| e.flatten().count() > 0).unwrap_or(false)
    };
    let candidates = [
        p.join("Sessions"),
        p.join("Default").join("Sessions"),
        p.to_path_buf(), // caller pointed straight at .../Default/Sessions
    ];
    for c in &candidates {
        if is_sessions_dir(c) {
            return Ok(c.to_path_buf());
        }
    }
    // Fall back to any <profile>/Sessions under the given dir.
    if let Ok(rd) = fs::read_dir(p) {
        for entry in rd.flatten() {
            let cand = entry.path().join("Sessions");
            if cand.is_dir() {
                return Ok(cand);
            }
        }
    }
    Err(format!(
        "no Sessions/ directory found at {input} (expected a Chrome profile dir, e.g. .../User Data/Default)"
    ))
}

fn md_escape(s: &str) -> String {
    s.replace('|', "\\|")
}

fn main() {
    let mut args = env::args().skip(1);
    let input = match args.next() {
        Some(d) => d,
        None => {
            eprintln!(
                "usage: snss-dump <chrome-user-data-or-profile-dir> [current|last|recently-closed|all]"
            );
            std::process::exit(2);
        }
    };
    let which = args.next().unwrap_or_else(|| "current".to_string());

    let sessions_dir = match resolve_sessions_dir(&input) {
        Ok(d) => d,
        Err(e) => {
            eprintln!("{e}");
            std::process::exit(1);
        }
    };

    let store = match SessionStore::open_dir(&sessions_dir) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("failed to open {}: {e}", sessions_dir.display());
            std::process::exit(1);
        }
    };

    let mut matched = false;
    for src in store.sources() {
        let include = match (which.as_str(), src.kind) {
            ("all", _) => true,
            ("current", SourceKind::Current) => true,
            ("last", SourceKind::Last) => true,
            ("recently-closed", SourceKind::RecentlyClosed) => true,
            _ => false,
        };
        if !include {
            continue;
        }
        matched = true;
        println!("## {} — {}", src.kind.label(), src.path.display());
        println!();
        for w in &src.windows {
            println!("### Window {} ({})", w.id, w.tabs.len() as u32);
            println!();
            println!("| # | Title | URL |");
            println!("|---|-------|-----|");
            for (i, tab) in w.tabs.iter().enumerate() {
                let nav = tab.current_nav();
                let pinned = if tab.pinned { " [pinned]" } else { "" };
                let title = if nav.title.is_empty() {
                    String::new()
                } else {
                    format!("{}{}", nav.title, pinned)
                };
                println!(
                    "| {} | {} | {} |",
                    i + 1,
                    md_escape(&title),
                    md_escape(&nav.url),
                );
            }
            println!();
        }
    }
    for w in store.warnings() {
        eprintln!("warning: {w:?}");
    }
    if !matched {
        eprintln!("no source matched '{which}' (available: {})",
            store.sources().iter().map(|s| s.kind.label()).collect::<Vec<_>>().join(", "));
        std::process::exit(1);
    }
}
