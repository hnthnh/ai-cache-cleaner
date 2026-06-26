use clap::{Parser, Subcommand};
use colored::*;
use dialoguer::Confirm;
use std::path::Path;

use crate::cleaner::{clean_items_interactive, clean_path};
use crate::display::{format_size, print_banner, run_scan, CleanerTheme};
use crate::error::Result;
use crate::locations::get_scan_locations;
use crate::scanner::{discover_sub_items, get_folder_size};

/// CLI Struct for parsing command line arguments.
#[derive(Parser)]
#[command(name = "ai-cache-cleaner")]
#[command(about = "🧹 AI Cache Cleaner: Clean up cache, weights, and logs (HuggingFace, Ollama, PyTorch, Pip, Cursor, etc.)", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

/// Commands enum representing supported CLI operations.
#[derive(Subcommand)]
pub enum Commands {
    /// Scan cache directories and calculate sizes.
    Scan,
    /// Clean cache files with interactive sub-item selection.
    Clean {
        /// Optional target to clean (all, hf, ollama, pip, torch, cursor, or a custom directory path).
        target: Option<String>,
        /// Do not delete any files, just print what would be deleted.
        #[arg(short = 'n', long = "dry-run")]
        dry_run: bool,
    },
}

/// Dispatches CLI commands to their respective actions and runs the clean workflows.
pub fn run() -> Result<()> {
    let cli = Cli::parse();
    let locations = get_scan_locations();

    match &cli.command {
        None => {
            run_scan(&locations);
            println!("{}", "Use --help to see all available commands".dimmed());
        }
        Some(Commands::Scan) => {
            run_scan(&locations);
        }
        Some(Commands::Clean { target, dry_run }) => {
            let dry_run = *dry_run;
            match target {
                None => {
                    print_banner();
                    let scan_label = if dry_run {
                        "[DRY RUN] Scanning"
                    } else {
                        "Scanning"
                    };
                    println!(
                        "\n⚡ {}",
                        format!("{} all directories to list cleanable items...", scan_label)
                            .bold()
                            .yellow()
                    );
                    let mut all_sub_items = Vec::new();

                    for loc in &locations {
                        if loc.path.exists() {
                            let items = discover_sub_items(loc.key, loc.name, &loc.path);
                            all_sub_items.extend(items);
                        }
                    }

                    clean_items_interactive(all_sub_items, dry_run)?;
                }
                Some(val) if val == "all" => {
                    print_banner();

                    let prompt_prefix = if dry_run { "[DRY RUN] " } else { "" };
                    let confirm_prompt = format!(
                        "{}Are you sure you want to clean ALL cached directories?",
                        prompt_prefix.bold().yellow()
                    );

                    // Always ask for confirmation before deleting everything, even in dry-run
                    let confirmed = Confirm::with_theme(&CleanerTheme)
                        .with_prompt(confirm_prompt)
                        .default(false)
                        .interact()?;

                    if confirmed {
                        let action_prefix = if dry_run {
                            "[DRY RUN] Would clean"
                        } else {
                            "Cleaning"
                        };
                        println!(
                            "\n🧹 {} all cached directories...",
                            action_prefix.bold().yellow()
                        );
                        let mut total_freed = 0;
                        let mut has_ollama = false;

                        for loc in &locations {
                            if loc.path.exists() {
                                let size = get_folder_size(&loc.path);
                                if size > 0 {
                                    if dry_run {
                                        println!(
                                            "  🗑️  [DRY RUN] Would clean: {} ({})",
                                            loc.name,
                                            format_size(size)
                                        );
                                        total_freed += size;
                                        if loc.key == "ollama" {
                                            has_ollama = true;
                                        }
                                    } else {
                                        match clean_path(&loc.path) {
                                            Ok(_) => {
                                                println!(
                                                    "  ✅ Cleaned {} ({})",
                                                    loc.name,
                                                    format_size(size)
                                                );
                                                total_freed += size;
                                                if loc.key == "ollama" {
                                                    has_ollama = true;
                                                }
                                            }
                                            Err(e) => {
                                                println!("  ❌ Failed to clean {}: {}", loc.name, e)
                                            }
                                        }
                                    }
                                }
                            }
                        }

                        let done_label = if dry_run { "Would free" } else { "freed!" };
                        println!(
                            "\n✨ {} {}",
                            format_size(total_freed).green().bold(),
                            done_label
                        );

                        if has_ollama {
                            println!("\n💡 {}", "Note for Ollama: Run 'ollama gc' in your terminal to prune actual model weights (blobs) from disk.".yellow());
                        }
                    } else {
                        println!("\n❌ {}", "Operation cancelled by user.".yellow());
                    }
                }
                Some(val) => {
                    print_banner();
                    // Specific target (hf, ollama, pip, torch, cursor)
                    if let Some(loc) = locations.iter().find(|l| l.key == val) {
                        let scan_label = if dry_run {
                            "[DRY RUN] Scanning"
                        } else {
                            "Scanning"
                        };
                        println!(
                            "\n⚡ {}",
                            format!(
                                "{} {} directory to list cleanable items...",
                                scan_label, loc.name
                            )
                            .bold()
                            .yellow()
                        );
                        let items = discover_sub_items(loc.key, loc.name, &loc.path);
                        clean_items_interactive(items, dry_run)?;
                    } else {
                        // Custom path clean
                        let custom_path = Path::new(val);
                        if !custom_path.exists() {
                            println!(
                                "❌ Error: Target '{}' is not a valid cache key or directory path.",
                                val
                            );
                            std::process::exit(1);
                        }

                        let size = get_folder_size(custom_path);
                        let prompt_prefix = if dry_run { "[DRY RUN] " } else { "" };
                        let prompt = format!(
                            "{}Are you sure you want to clean custom directory '{}' ({})?",
                            prompt_prefix.bold().yellow(),
                            val,
                            format_size(size)
                        );

                        let confirmed = Confirm::with_theme(&CleanerTheme)
                            .with_prompt(prompt)
                            .default(false)
                            .interact()?;

                        if confirmed {
                            if dry_run {
                                println!(
                                    "✅ [DRY RUN] Would clean custom path: {} ({})",
                                    val.bold().cyan(),
                                    format_size(size).bold().green()
                                );
                            } else {
                                match clean_path(custom_path) {
                                    Ok(_) => println!(
                                        "✅ {}",
                                        "Successfully cleaned custom path".green()
                                    ),
                                    Err(e) => println!("❌ Error: {}", e),
                                }
                            }
                        } else {
                            println!("\n❌ {}", "Operation cancelled by user.".yellow());
                        }
                    }
                }
            }
        }
    }
    Ok(())
}
