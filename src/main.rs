use mcfetch::{stacks::Stacks, stats::Stats};

fn main() {
    let file_path = "/home/prezes/.minecraft/saves/SIGMA SIGMA BOY/stats/91b5062c-c44a-4aed-b997-d28290b39e15.json";
    let stats = Stats::from_file_path(file_path).unwrap();
    // println!("{:?}", stats.crafted);
    if let Some(most_crafted) = stats.most_crafted() {
        println!(
            "Most crafted item: {}, crafted {} times ({:.2} stacks)",
            most_crafted.name,
            most_crafted.count,
            most_crafted.count.stacks()
        );
    }
    println!(
        "Total crafted items: {} ({:.2} stacks)",
        stats.total_crafted(),
        stats.total_crafted().stacks()
    );
    if let Some(most_broken) = stats.most_broken() {
        println!(
            "Most broken item: {}, broken {} times ({:.2} stacks)",
            most_broken.name,
            most_broken.count,
            most_broken.count.stacks()
        );
    }
    println!(
        "Total broken items: {} ({:.2} stacks)",
        stats.total_broken(),
        stats.total_broken().stacks()
    );
}
