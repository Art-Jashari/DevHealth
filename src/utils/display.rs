//! Display utilities for enhanced output formatting
//!
//! This module provides utilities for creating beautiful, colorized terminal output
//! with consistent formatting, progress indicators, and visual hierarchy.

use colored::*;

/// Creates a styled header with optional emoji and color
pub fn header(title: &str, emoji: &str, color: Color) -> String {
    format!("{} {}", emoji, title.color(color).bold())
}

/// Creates a styled section divider
pub fn section_divider(title: &str) -> String {
    let divider = "─".repeat(50);
    format!("\n{}\n{} {}\n{}", 
        divider.bright_black(),
        "▶".bright_blue().bold(), 
        title.bright_white().bold(),
        divider.bright_black()
    )
}

/// Creates a summary box with statistics
pub fn summary_box(items: &[(&str, String)]) -> String {
    let mut result = String::new();
    result.push_str(&"┌─ Summary ─────────────────────────────────────────┐\n".bright_black().to_string());
    
    for (label, value) in items {
        result.push_str(&format!("│ {:<20} {} {}\n", 
            label.bright_blue(),
            "│".bright_black(),
            value.bright_white().bold()
        ));
    }
    
    result.push_str(&"└───────────────────────────────────────────────────┘\n".bright_black().to_string());
    result
}

/// Creates a progress bar representation
pub fn progress_bar(current: usize, total: usize, width: usize) -> String {
    if total == 0 {
        return "".to_string();
    }
    
    let filled = (current * width) / total;
    let empty = width - filled;
    
    format!("[{}{}] {}/{}", 
        "█".repeat(filled).bright_green(),
        "░".repeat(empty).bright_black(),
        current.to_string().bright_white().bold(),
        total.to_string().bright_black()
    )
}

/// Creates a status indicator with appropriate colors
pub fn status_indicator(status: &str, is_good: bool) -> String {
    let (symbol, color) = match (status, is_good) {
        (_, true) => ("✓", Color::BrightGreen),
        (_, false) => ("✗", Color::BrightRed),
    };
    
    format!("{} {}", symbol.color(color).bold(), status.color(color))
}

/// Creates a tree-like structure indicator
pub fn tree_item(content: &str, is_last: bool, level: usize) -> String {
    let indent = "  ".repeat(level);
    let connector = if is_last { "└─" } else { "├─" };
    
    format!("{}{} {}", 
        indent.bright_black(),
        connector.bright_black(),
        content
    )
}

/// Creates a badge for dependency types or categories
pub fn badge(text: &str, badge_type: BadgeType) -> String {
    let (bg_color, text_color) = match badge_type {
        BadgeType::Runtime => (Color::BrightGreen, Color::Black),
        BadgeType::Dev => (Color::BrightYellow, Color::Black),
        BadgeType::Build => (Color::BrightBlue, Color::White),
        BadgeType::Optional => (Color::BrightMagenta, Color::White),
        BadgeType::Error => (Color::BrightRed, Color::White),
        BadgeType::Warning => (Color::Yellow, Color::Black),
        BadgeType::Info => (Color::Cyan, Color::Black),
    };
    
    format!(" {} ", text.color(text_color).on_color(bg_color).bold())
}

/// Badge types for different categories
pub enum BadgeType {
    Runtime,
    Dev,
    Build,
    Optional,
    Error,
    Warning,
    Info,
}

/// Creates a file path display with proper highlighting
pub fn file_path(path: &str) -> String {
    path.bright_black().italic().to_string()
}

/// Creates an ecosystem icon with color
pub fn ecosystem_icon(ecosystem: &str) -> String {
    match ecosystem.to_lowercase().as_str() {
        "rust" => "🦀".to_string(),
        "node.js" | "nodejs" => "📦".to_string(),
        "python" => "🐍".to_string(),
        "go" => "🐹".to_string(),
        _ => "📄".to_string(),
    }
}

/// Creates a version display with proper formatting
pub fn version_display(name: &str, version: &str, is_latest: Option<bool>) -> String {
    let name_colored = name.bright_white().bold();
    let version_colored = version.bright_green();
    
    match is_latest {
        Some(true) => format!("{} {} {}", name_colored, version_colored, "✓".bright_green()),
        Some(false) => format!("{} {} {}", name_colored, version_colored, "⚠".yellow()),
        None => format!("{} {}", name_colored, version_colored),
    }
}

/// Creates a table-like layout for dependency information
pub fn dependency_table_row(name: &str, version: &str, dep_type: &str, source: &str) -> String {
    format!("│ {:<25} │ {:<12} │ {:<8} │ {:<20} │",
        name.bright_white().bold(),
        version.bright_green(),
        dep_type.color(match dep_type {
            "runtime" => Color::BrightGreen,
            "dev" => Color::BrightYellow,
            "build" => Color::BrightBlue,
            _ => Color::BrightMagenta,
        }),
        source.bright_black().italic()
    )
}

/// Creates table header
pub fn dependency_table_header() -> String {
    let header = format!("┌─{:─<25}─┬─{:─<12}─┬─{:─<8}─┬─{:─<20}─┐",
        "─", "─", "─", "─");
    let titles = format!("│ {:<25} │ {:<12} │ {:<8} │ {:<20} │",
        "Package".bright_blue().bold(),
        "Version".bright_blue().bold(),
        "Type".bright_blue().bold(),
        "Source".bright_blue().bold()
    );
    let separator = format!("├─{:─<25}─┼─{:─<12}─┼─{:─<8}─┼─{:─<20}─┤",
        "─", "─", "─", "─");
    
    format!("{}\n{}\n{}", header.bright_black(), titles, separator.bright_black())
}

/// Creates table footer
pub fn dependency_table_footer() -> String {
    format!("└─{:─<25}─┴─{:─<12}─┴─{:─<8}─┴─{:─<20}─┘",
        "─", "─", "─", "─").bright_black().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_header_with_emoji_and_color() {
        let result = header("Test Header", "🔍", Color::Blue);
        assert!(result.contains("Test Header"));
        assert!(result.contains("🔍"));
    }

    #[test]
    fn creates_progress_bar() {
        let result = progress_bar(3, 10, 20);
        assert!(result.contains("["));
        assert!(result.contains("]"));
        assert!(result.contains("3/10"));
    }

    #[test]
    fn handles_zero_total_in_progress_bar() {
        let result = progress_bar(0, 0, 20);
        assert_eq!(result, "");
    }

    #[test]
    fn creates_ecosystem_icons() {
        assert_eq!(ecosystem_icon("rust"), "🦀");
        assert_eq!(ecosystem_icon("Node.js"), "📦");
        assert_eq!(ecosystem_icon("python"), "🐍");
        assert_eq!(ecosystem_icon("go"), "🐹");
        assert_eq!(ecosystem_icon("unknown"), "📄");
    }
}
