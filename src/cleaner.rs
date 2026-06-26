use crate::display::{format_size, CleanerTheme};
use crate::error::Result;
use crate::scanner::SubItem;
use colored::*;
use dialoguer::{Confirm, MultiSelect};
use std::fs;
use std::path::Path;

/// Deletes the file or directory at the specified path.
pub fn clean_path(path: &Path) -> std::io::Result<()> {
    if !path.exists() {
        return Ok(());
    }
    if path.is_file() {
        fs::remove_file(path)?;
    } else {
        fs::remove_dir_all(path)?;
    }
    Ok(())
}

/// Runs the interactive selection and deletion workflow for a collection of SubItems.
pub fn clean_items_interactive(mut sub_items: Vec<SubItem>, dry_run: bool) -> Result<()> {
    if sub_items.is_empty() {
        println!(
            "🎉 {}",
            "No cleanable sub-items found in the scanned directories!".green()
        );
        return Ok(());
    }

    // Sort descending by size (largest first)
    sub_items.sort_by_key(|item| std::cmp::Reverse(item.size));

    println!("\n📋 {} found (sorted by size):", "Sub-items".bold().cyan());
    let items_labels: Vec<String> = sub_items
        .iter()
        .map(|item| {
            format!(
                "[{}] {} -> Size: {}",
                item.category.bold().yellow(),
                item.name.cyan(),
                format_size(item.size).bold()
            )
        })
        .collect();

    // Default: select all items
    let defaults = vec![true; sub_items.len()];

    // Interactive prompt for selection
    let selections = MultiSelect::with_theme(&CleanerTheme)
        .with_prompt("Select items to DELETE (Space to toggle, Enter to confirm)")
        .items(&items_labels)
        .defaults(&defaults)
        .interact()?;

    if selections.is_empty() {
        println!("\n🎉 {}", "No items selected. Nothing deleted.".green());
        return Ok(());
    }

    println!();
    let prompt_prefix = if dry_run { "[DRY RUN] " } else { "" };
    let confirm_prompt = format!(
        "{}Are you sure you want to permanently delete the {} selected items?",
        prompt_prefix.bold().yellow(),
        selections.len()
    );

    // Interactive confirmation prompt
    let confirmed = Confirm::with_theme(&CleanerTheme)
        .with_prompt(confirm_prompt)
        .default(false)
        .interact()?;

    if confirmed {
        let clean_prefix = if dry_run {
            "[DRY RUN] Would clean..."
        } else {
            "Cleaning..."
        };
        println!("\n🧹 {}", clean_prefix.bold().yellow());
        let mut freed_bytes = 0;
        let mut has_ollama = false;

        for idx in selections {
            let item = &sub_items[idx];
            if dry_run {
                println!(
                    "   🗑️  [DRY RUN] Would remove: {} -> Category: {}, Size: {}",
                    item.name.red(),
                    item.category.yellow(),
                    format_size(item.size).bold()
                );
                freed_bytes += item.size;
                if item.key == "ollama" {
                    has_ollama = true;
                }
            } else {
                match clean_path(&item.path) {
                    Ok(_) => {
                        println!(
                            "   🗑️  Removed: {} -> {}",
                            item.name.red(),
                            "DELETED".green().bold()
                        );
                        freed_bytes += item.size;
                        if item.key == "ollama" {
                            has_ollama = true;
                        }
                    }
                    Err(e) => {
                        println!(
                            "   🗑️  Failed to remove: {} -> {} ({})",
                            item.name.red(),
                            "FAILED".red().bold(),
                            e
                        );
                    }
                }
            }
        }

        let space_saved_label = if dry_run { "Would free" } else { "freed!" };
        println!(
            "\n✨ {} {}",
            format_size(freed_bytes).green().bold(),
            space_saved_label
        );

        if has_ollama {
            println!("\n💡 {}", "Note for Ollama: Run 'ollama gc' in your terminal to prune actual model weights (blobs) from disk.".yellow());
        }
    } else {
        println!("\n❌ {}", "Deletion cancelled by user.".yellow());
    }

    Ok(())
}
