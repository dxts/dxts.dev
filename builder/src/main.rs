//! Renders the site from `resume.yaml`:
//!   templates/index.html -> index.html  (screen)
//!   templates/print.html -> print.html  (A4 layout, printed to resume.pdf by `make pdf`)

use std::{error::Error, fs, path::Path};

use tera::{Context, Tera};

const TEMPLATES: &[&str] = &["index.html", "print.html"];

fn main() -> Result<(), Box<dyn Error>> {
    // Repo root is the parent of the builder/ crate, regardless of cwd.
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("builder crate must live inside the repo")
        .to_path_buf();

    let yaml = fs::read_to_string(root.join("resume.yaml"))?;
    let mut data: serde_json::Value = yaml_serde::from_str(&yaml)?;
    normalize(&mut data);

    let mut ctx = Context::from_serialize(&data)?;
    let now = chrono::Local::now();
    ctx.insert("last_updated", &now.format("%B %-d, %Y").to_string());
    ctx.insert("last_updated_iso", &now.format("%Y-%m-%d").to_string());

    let mut tera = Tera::new();
    for name in TEMPLATES {
        let src = fs::read_to_string(root.join("templates").join(name))?;
        tera.add_raw_template(name, &src)?;
    }

    for name in TEMPLATES {
        let rendered = tera.render(name, &ctx)?;
        let out = root.join(name);
        fs::write(&out, rendered)?;
        println!("wrote {}", out.display());
    }

    Ok(())
}

/// Templates always iterate paragraphs/highlights as lists of lines, so
/// wrap any bare string in a single-element list. This lets the YAML stay
/// natural: plain strings for the common case, lists only when an item
/// needs explicit line breaks.
fn normalize(data: &mut serde_json::Value) {
    if let Some(about) = data.get_mut("about") {
        listify_items(about);
    }
    for key in ["work", "education"] {
        let Some(entries) = data.get_mut(key).and_then(|v| v.as_array_mut()) else {
            continue;
        };
        for entry in entries {
            if let Some(highlights) = entry.get_mut("highlights") {
                listify_items(highlights);
            }
        }
    }
}

fn listify_items(value: &mut serde_json::Value) {
    let Some(items) = value.as_array_mut() else {
        return;
    };
    for item in items {
        if item.is_string() {
            *item = serde_json::Value::Array(vec![item.take()]);
        }
    }
}
