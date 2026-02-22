const CREEPER: &[&str] = &[
    "⣾⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣷  ",
    "⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿  ",
    "⣿⣿⣿⡟⠉⠉⠉⠉⢻⣿⣿⣿⣿⡟⠉⠉⠉⠉⢻⣿⣿⣿  ",
    "⣿⣿⣿⡇⠀⠀⠀⠀⢸⣿⣿⣿⣿⡇⠀⠀⠀⠀⢸⣿⣿⣿  ",
    "⣿⣿⣿⣇⣀⣀⣀⣀⡸⠿⠿⠿⠿⢇⣀⣀⣀⣀⣸⣿⣿⣿  ",
    "⣿⣿⣿⣿⣿⣿⣿⣿⡇⠀⠀⠀⠀⢸⣿⣿⣿⣿⣿⣿⣿⣿  ",
    "⣿⣿⣿⣿⣿⣿⠉⠉⠁⠀⠀⠀⠀⠈⠉⠉⣿⣿⣿⣿⣿⣿  ",
    "⣿⣿⣿⣿⣿⣿⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣿⣿⣿⣿⣿⣿  ",
    "⣿⣿⣿⣿⣿⣿⠀⠀⣶⣶⣶⣶⣶⣶⠀⠀⣿⣿⣿⣿⣿⣿  ",
    "⣿⣿⣿⣿⣿⣿⣶⣾⣿⣿⣿⣿⣿⣿⣷⣶⣿⣿⣿⣿⣿⣿  ",
    "⢿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡿  ",
];

const GREEN: &str = "\x1b[32m";
const BOLD: &str = "\x1b[1m";
const RESET: &str = "\x1b[0m";

pub fn render(stats: &[String]) -> String {
    let art_width = CREEPER.iter().map(|l| l.chars().count()).max().unwrap_or(0);
    let total_lines = CREEPER.len().max(stats.len());
    let mut out = String::new();

    for i in 0..total_lines {
        let art_line = if i < CREEPER.len() { CREEPER[i] } else { "" };
        let padding = art_width.saturating_sub(art_line.chars().count());
        out.push_str(&format!("{GREEN}{art_line}{}{RESET}", " ".repeat(padding)));

        if let Some(stat) = stats.get(i) {
            out.push_str(stat);
        }
        out.push('\n');
    }

    out
}

pub fn title_line(name: &str) -> String {
    format!("{BOLD}{name}{RESET}")
}

pub fn separator(len: usize) -> String {
    "─".repeat(len)
}

pub fn stat_line(label: &str, value: &str) -> String {
    format!("{BOLD}{label}:{RESET} {value}")
}
