use crate::locations::CacheLocation;
use crate::scanner::get_folder_size;
use colored::*;

/// Formats a byte size into a human-readable string representation (B, KB, MB, GB, TB).
pub fn format_size(bytes: u64) -> String {
    let units = ["B", "KB", "MB", "GB", "TB"];
    let mut size = bytes as f64;
    let mut unit_idx = 0;

    while size >= 1024.0 && unit_idx < units.len() - 1 {
        size /= 1024.0;
        unit_idx += 1;
    }
    format!("{:.2} {}", size, units[unit_idx])
}

/// Prints the application banner to standard output.
pub fn print_banner() {
    println!(
        "{}",
        "┌────────────────────────────────────────────────────────┐"
            .cyan()
            .dimmed()
    );
    println!(
        "│                {}               │",
        "🧹 AI Cache Cleaner 🧹".bold().cyan()
    );
    println!(
        "│       {}       │",
        "Blazing Fast system garbage collector".dimmed()
    );
    println!(
        "{}",
        "└────────────────────────────────────────────────────────┘"
            .cyan()
            .dimmed()
    );
}

/// Scans the target locations, calculating and printing their sizes and overall claimable space.
pub fn run_scan(locations: &[CacheLocation]) {
    print_banner();
    println!(
        "\n🔍 {}",
        "Scanning system cache directories...".bold().cyan()
    );
    let mut total_bytes = 0;

    for loc in locations {
        let exists = loc.path.exists();
        let size = if exists {
            get_folder_size(&loc.path)
        } else {
            0
        };
        total_bytes += size;

        let status_symbol = if exists {
            "📂".green()
        } else {
            "❌ (Not found)".red()
        };
        println!(
            "  {} {} [{}]:",
            status_symbol,
            loc.name.bold(),
            loc.key.yellow()
        );
        println!("     Path: {}", loc.path.display().to_string().dimmed());
        println!("     Size: {}", format_size(size).bold().cyan());
        println!("{}", "-".repeat(58).dimmed());
    }

    println!(
        "\n✨ {} {}\n",
        "Total Claimable Space:".bold(),
        format_size(total_bytes).green().bold()
    );
}

/// Custom theme for Dialoguer to resolve dimmed select options and display checkbox indicators cleanly.
pub struct CleanerTheme;

impl dialoguer::theme::Theme for CleanerTheme {
    fn format_multi_select_prompt(
        &self,
        f: &mut dyn std::fmt::Write,
        prompt: &str,
    ) -> std::fmt::Result {
        write!(f, "❓ {}", prompt)
    }

    fn format_multi_select_prompt_selection(
        &self,
        f: &mut dyn std::fmt::Write,
        prompt: &str,
        selections: &[&str],
    ) -> std::fmt::Result {
        write!(f, "{} ... Selected: {}", prompt, selections.join(", "))
    }

    fn format_multi_select_prompt_item(
        &self,
        f: &mut dyn std::fmt::Write,
        text: &str,
        checked: bool,
        active: bool,
    ) -> std::fmt::Result {
        let marker = if checked { "[x] -- " } else { "[ ] -- " };

        if active {
            // Highlight the active cursor line and checkbox indicator, but preserve original text colors
            write!(
                f,
                " {} {}{}",
                ">".yellow().bold(),
                marker.yellow().bold(),
                text
            )
        } else {
            // Render inactive items clearly without dimming the main text
            write!(f, "   {}{}", marker, text)
        }
    }

    fn format_confirm_prompt(
        &self,
        f: &mut dyn std::fmt::Write,
        prompt: &str,
        default: Option<bool>,
    ) -> std::fmt::Result {
        let options_str = match default {
            Some(true) => " [Y/n]",
            Some(false) => " [y/N]",
            None => " [y/n]",
        };
        write!(f, "❓ {}{}", prompt, options_str)
    }

    fn format_confirm_prompt_selection(
        &self,
        f: &mut dyn std::fmt::Write,
        prompt: &str,
        selection: Option<bool>,
    ) -> std::fmt::Result {
        let selection_str = match selection {
            Some(true) => "yes",
            Some(false) => "no",
            None => "",
        };
        write!(f, "{} ... {}", prompt, selection_str)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_size() {
        assert_eq!(format_size(0), "0.00 B");
        assert_eq!(format_size(1023), "1023.00 B");
        assert_eq!(format_size(1024), "1.00 KB");
        assert_eq!(format_size(1024 * 1024), "1.00 MB");
        assert_eq!(format_size(1024 * 1024 * 1024), "1.00 GB");
    }
}
