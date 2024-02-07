use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use std::{
    fs::OpenOptions,
    io::{self, Write},
};
use anyhow::Result;

#[derive(Deserialize, Serialize)]
pub struct Project {
    pub repo: String,
    pub name: String,
    pub folder: String,
    pub tags: Vec<String>,
}

pub fn read_projects(projects_file_path: &Path) -> Result<Vec<Project>> {
    if projects_file_path.exists() {
        Ok(serde_json::from_str(
            &fs::read(projects_file_path)?
                .iter()
                .map(|i| *i as char)
                .collect::<String>(),
        )?)
    } else {
        Ok(vec![])
    }
}

pub fn save_projects(projects: &Vec<Project>, projects_file_path: &Path) -> Result<()> {
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(projects_file_path)?;

    file.write_all(
        &serde_json::to_string_pretty(&projects)?
            .chars()
            .map(|c| c as u8)
            .collect::<Vec<u8>>(),
    )?;

    Ok(())
}
