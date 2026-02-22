use convert_case::{Case, Casing};
use std::{collections::HashMap, fs::read_to_string};

use anyhow::anyhow;
use serde::Deserialize;
use serde_json::Value;

#[derive(Deserialize)]
pub struct Stats {
    pub broken: Vec<ItemEntry>,
    pub crafted: Vec<ItemEntry>,
    pub dropped: Vec<ItemEntry>,
    pub used: Vec<ItemEntry>,
    pub killed: Vec<ItemEntry>,
    pub mined: Vec<ItemEntry>,
    pub picked_up: Vec<ItemEntry>,
    pub custom: Vec<ItemEntry>,
}

impl Stats {
    fn parse_section(section: Value) -> anyhow::Result<Vec<ItemEntry>> {
        let section: HashMap<String, u32> = serde_json::from_value(section)?;
        let section = section
            .iter()
            .filter_map(|(name, count)| {
                name.strip_prefix("minecraft:").map(|name| ItemEntry {
                    name: name.to_case(Case::Title),
                    count: *count,
                })
            })
            .collect::<Vec<_>>();

        Ok(section)
    }

    fn get_section(stats: &Value, key: &str) -> anyhow::Result<Vec<ItemEntry>> {
        match stats.get(key) {
            Some(section) => Self::parse_section(section.clone()),
            None => Ok(Vec::new()),
        }
    }

    pub fn from_file_path(file_path: &str) -> anyhow::Result<Self> {
        let value = read_to_string(file_path)?;
        let value: Value = serde_json::from_str(&value)?;
        let stats = value
            .get("stats")
            .ok_or(anyhow!("Couldn't get stats key"))?;

        Ok(Stats {
            broken: Self::get_section(stats, "minecraft:broken")?,
            crafted: Self::get_section(stats, "minecraft:crafted")?,
            dropped: Self::get_section(stats, "minecraft:dropped")?,
            used: Self::get_section(stats, "minecraft:used")?,
            killed: Self::get_section(stats, "minecraft:killed")?,
            mined: Self::get_section(stats, "minecraft:mined")?,
            picked_up: Self::get_section(stats, "minecraft:picked_up")?,
            custom: Self::get_section(stats, "minecraft:custom")?,
        })
    }

    pub fn most(section: &[ItemEntry]) -> Option<&ItemEntry> {
        section.iter().max_by_key(|e| e.count)
    }

    pub fn total(section: &[ItemEntry]) -> u32 {
        section.iter().map(|e| e.count).sum()
    }

    pub fn custom_stat(&self, name: &str) -> Option<u32> {
        self.custom
            .iter()
            .find(|e| e.name == name.to_case(Case::Title))
            .map(|e| e.count)
    }
}

#[derive(Debug, Deserialize)]
pub struct ItemEntry {
    pub name: String,
    pub count: u32,
}
