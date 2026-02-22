use std::path::PathBuf;

use mcfetch::{config, display, stacks::Stacks, stats::Stats, worlds};

fn run_config() -> anyhow::Result<()> {
    let worlds = worlds::discover_worlds();
    if worlds.is_empty() {
        anyhow::bail!("No Minecraft worlds found");
    }

    let mut select = cliclack::select("Select a Minecraft world");
    for world in &worlds {
        select = select.item(&world.path, &world.name, "");
    }
    let selected: &PathBuf = select.interact()?;

    config::save(&config::Config {
        world_path: Some(selected.to_string_lossy().into_owned()),
    })?;

    println!("Saved world: {}", selected.display());
    Ok(())
}

fn run_display() -> anyhow::Result<()> {
    let cfg = config::load();

    let world_path = if let Some(ref path) = cfg.world_path {
        PathBuf::from(path)
    } else {
        let worlds = worlds::discover_worlds();
        worlds::latest_world(&worlds)
            .map(|w| w.path.clone())
            .ok_or_else(|| anyhow::anyhow!("No Minecraft worlds found"))?
    };

    let stats_file = worlds::find_stats_file(&world_path)
        .ok_or_else(|| anyhow::anyhow!("No stats file found in {}", world_path.display()))?;

    let stats = Stats::from_file_path(stats_file.to_str().unwrap())?;
    let world_name = world_path
        .file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_else(|| "Unknown".into());

    let title = format!("mcfetch - {world_name}");
    let mut lines = vec![
        display::title_line(&title),
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
    Ok(())
}

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = std::env::args().collect();

    if args.get(1).is_some_and(|a| a == "config") {
        run_config()
    } else {
        run_display()
    }
}
