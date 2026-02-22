use mcfetch::{display, stacks::Stacks, stats::Stats};

fn main() {
    let file_path = "/home/prezes/.minecraft/saves/SIGMA SIGMA BOY/stats/91b5062c-c44a-4aed-b997-d28290b39e15.json";
    let stats = Stats::from_file_path(file_path).unwrap();

    let title = "mcfetch";
    let mut lines = vec![
        display::title_line(title),
        display::separator(title.len() + 16),
    ];

    if let Some(most_crafted) = stats.most_crafted() {
        lines.push(display::stat_line(
            "Most Crafted",
            &format!(
                "{} ({}x, {:.2} stacks)",
                most_crafted.name,
                most_crafted.count,
                most_crafted.count.stacks()
            ),
        ));
    }
    lines.push(display::stat_line(
        "Total Crafted",
        &format!(
            "{} ({:.2} stacks)",
            stats.total_crafted(),
            stats.total_crafted().stacks()
        ),
    ));

    if let Some(most_broken) = stats.most_broken() {
        lines.push(display::stat_line(
            "Most Broken",
            &format!(
                "{} ({}x, {:.2} stacks)",
                most_broken.name,
                most_broken.count,
                most_broken.count.stacks()
            ),
        ));
    }
    lines.push(display::stat_line(
        "Total Broken",
        &format!(
            "{} ({:.2} stacks)",
            stats.total_broken(),
            stats.total_broken().stacks()
        ),
    ));

    print!("\n{}", display::render(&lines));
}
