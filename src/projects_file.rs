use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use std::{fs::OpenOptions, io::Write};

#[derive(Deserialize, Serialize)]
pub struct Project {
    pub repo: String,
    pub name: String,
    pub folder: String,
    pub tags: Vec<String>,
}

pub fn read_projects(projects_file_path: &Path) -> Result<Vec<Project>> {
    if projects_file_path.exists() {
        let buffer =
            &fs::read(projects_file_path).context("Failed to read projects from project file")?;

        let json = std::str::from_utf8(buffer).context(
            "Failed to convert to read config file as utf8. Please ensure it isn't corrupted",
        )?;

        let projects = serde_json::from_str(json)
            .context("Data in project file is not in the right format")?;

        Ok(projects)
    } else {
        Ok(vec![])
    }
}

pub fn get_project_position(projects: &Vec<Project>, name: String) -> Result<usize> {
    projects
        .iter()
        .position(|p| p.name == name)
        .context(format!("Failed to find project with name {name}"))
}

pub fn save_projects(projects: &Vec<Project>, projects_file_path: &Path) -> Result<()> {
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(projects_file_path)
        .context("Failed to open projects file for writing")?;

    file.write_all(
        &serde_json::to_string_pretty(&projects)
            .context("Failed to serialize data as a string")?
            .chars()
            .map(|c| c as u8)
            .collect::<Vec<u8>>(),
    )?;

    Ok(())
}
