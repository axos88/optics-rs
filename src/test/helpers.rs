use anyhow::Context;
use once_cell::sync::Lazy;
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;
use syn::File;
use syn::Item;
use syn::ItemMod;
use syn::token::Brace;

thread_local! {
  pub static CRATE_AST: Lazy<File> = Lazy::new(|| {
    let root_file = PathBuf::from("src/lib.rs");
    let base_dir = root_file.parent().unwrap();
    expand_mods_in_file(&root_file, base_dir).expect("Failed to expand crate AST")
  });
}

thread_local! {
  pub static EXPANDED_AST: Lazy<File> = Lazy::new(|| {

    let output = Command::new("cargo")
        .args(["expand", "--lib"])
        .output()
        .expect("Failed to run cargo expand, are you sure it's installed?");

    let expanded_src = String::from_utf8(output.stdout).expect("Invalid UTF-8");

    syn::parse_file(&expanded_src).expect("Failed to parse expanded source")
  })
}

/// Recursively resolve external `mod` declarations into inlined modules.
fn expand_mods_in_file(path: &Path, base_dir: &Path) -> anyhow::Result<File> {
    let src =
        fs::read_to_string(path).with_context(|| format!("Failed to read {}", path.display()))?;

    let mut file: File =
        syn::parse_file(&src).with_context(|| format!("Failed to parse {}", path.display()))?;

    expand_mods_in_items(&mut file.items, base_dir)?;

    Ok(file)
}

/// Recursively resolve external mods in a list of items.
fn expand_mods_in_items(items: &mut [Item], base_dir: &Path) -> anyhow::Result<()> {
    for item in items.iter_mut() {
        if let Item::Mod(mod_item) = item {
            expand_mod_item(mod_item, base_dir)?;
        }
    }
    Ok(())
}

/// Resolve an external mod declaration into an inlined module.
fn expand_mod_item(mod_item: &mut ItemMod, base_dir: &Path) -> anyhow::Result<()> {
    if mod_item.content.is_none() {
        let mod_name = mod_item.ident.to_string();
        let mod_path_rs = base_dir.join(format!("{mod_name}.rs"));
        let mod_path_modrs = base_dir.join(&mod_name).join("mod.rs");

        let mod_path = if mod_path_rs.exists() {
            mod_path_rs
        } else if mod_path_modrs.exists() {
            mod_path_modrs
        } else {
            panic!(
                "Could not find module file for mod {} in {}",
                mod_name,
                base_dir.display()
            );
        };

        let sub_base_dir = mod_path.parent().unwrap();
        let src = fs::read_to_string(&mod_path)
            .with_context(|| format!("Failed to read {}", mod_path.display()))?;

        let mut sub_file: File = syn::parse_file(&src)
            .with_context(|| format!("Failed to parse {}", mod_path.display()))?;

        // Recursively expand nested mods
        expand_mods_in_items(&mut sub_file.items, sub_base_dir)?;

        // Replace mod_item content with parsed items
        mod_item.content = Some((Brace::default(), sub_file.items));
    }

    // If already inlined, recurse into its content too
    if let Some((_brace, items)) = &mut mod_item.content {
        expand_mods_in_items(items, base_dir)?;
    }

    Ok(())
}
