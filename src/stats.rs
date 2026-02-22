use convert_case::{Case, Casing};
use itertools::Itertools;
use std::{collections::HashMap, fs::read_to_string};

use anyhow::anyhow;
use serde::Deserialize;
use serde_json::Value;

#[derive(Deserialize)]
pub struct Stats {
    pub broken: Vec<ItemEntry>,
    pub crafted: Vec<ItemEntry>,
}

impl Stats {
    pub fn parse_section(section: Value) -> anyhow::Result<Vec<ItemEntry>> {
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

    pub fn from_file_path(file_path: &str) -> anyhow::Result<Self> {
        let value = read_to_string(file_path)?;
        let value: Value = serde_json::from_str(&value)?;
        let value = value
            .get("stats")
            .ok_or(anyhow!("Couldn't get stats key"))?;

        let broken = match value.get("minecraft:broken") {
            Some(section) => Self::parse_section(section.clone())?,
            None => Vec::new(),
        };

        let crafted = match value.get("minecraft:crafted") {
            Some(section) => Self::parse_section(section.clone())?,
            None => Vec::new(),
        };

        Ok(Stats { broken, crafted })
    }

    pub fn most_crafted(&self) -> Option<&ItemEntry> {
        self.crafted
            .iter()
            .max_by_key(|item_entry| item_entry.count)
    }

    pub fn total_crafted(&self) -> u32 {
        self.crafted.iter().map(|item_entry| item_entry.count).sum()
    }

    pub fn most_broken(&self) -> Option<&ItemEntry> {
        self.broken
            .iter()
            .max_by_key(|item_entry| item_entry.count)
    }

    pub fn total_broken(&self) -> u32 {
        self.broken.iter().map(|item_entry| item_entry.count).sum()
    }
}

#[derive(Debug, Deserialize)]
pub struct ItemEntry {
    pub name: String,
    pub count: u32,
}
