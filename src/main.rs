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

    // Mined
    if let Some(top) = Stats::most(&stats.mined) {
        lines.push(display::stat_line(
            "Most Mined",
            &format!("{} ({}x)", top.name, top.count),
        ));
    }
    lines.push(display::stat_line(
        "Total Mined",
        &format!("{}", Stats::total(&stats.mined)),
    ));

    // Crafted
    if let Some(top) = Stats::most(&stats.crafted) {
        lines.push(display::stat_line(
            "Most Crafted",
            &format!(
                "{} ({}x, {:.2} stacks)",
                top.name,
                top.count,
                top.count.stacks()
            ),
        ));
    }

    // Used
    if let Some(top) = Stats::most(&stats.used) {
        lines.push(display::stat_line(
            "Most Used",
            &format!("{} ({}x)", top.name, top.count),
        ));
    }

    // Broken
    if let Some(top) = Stats::most(&stats.broken) {
        lines.push(display::stat_line(
            "Most Broken",
            &format!("{} ({}x)", top.name, top.count),
        ));
    }

    // Picked up
    if let Some(top) = Stats::most(&stats.picked_up) {
        lines.push(display::stat_line(
            "Most Picked Up",
            &format!(
                "{} ({}x, {:.2} stacks)",
                top.name,
                top.count,
                top.count.stacks()
            ),
        ));
    }

    // Dropped
    if let Some(top) = Stats::most(&stats.dropped) {
        lines.push(display::stat_line(
            "Most Dropped",
            &format!(
                "{} ({}x, {:.2} stacks)",
                top.name,
                top.count,
                top.count.stacks()
            ),
        ));
    }

    // Killed
    if let Some(top) = Stats::most(&stats.killed) {
        lines.push(display::stat_line(
            "Most Killed",
            &format!("{} ({}x)", top.name, top.count),
        ));
    }
    lines.push(display::stat_line(
        "Mob Kills",
        &format!("{}", Stats::total(&stats.killed)),
    ));

    // Custom stats
    if let Some(play_ticks) = stats.custom_stat("play_time") {
        let hours = play_ticks / 20 / 3600;
        let mins = (play_ticks / 20 % 3600) / 60;
        lines.push(display::stat_line(
            "Play Time",
            &format!("{}h {}m", hours, mins),
        ));
    }

    if let Some(walk_cm) = stats.custom_stat("walk_one_cm") {
        let km = walk_cm as f64 / 100_000.0;
        lines.push(display::stat_line(
            "Distance Walked",
            &format!("{:.2} km", km),
        ));
    }

    if let Some(deaths) = stats.custom_stat("deaths") {
        lines.push(display::stat_line("Deaths", &format!("{}", deaths)));
    }

    if let Some(jumps) = stats.custom_stat("jump") {
        lines.push(display::stat_line("Jumps", &format!("{}", jumps)));
    }

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
