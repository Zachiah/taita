use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use std::process::Command;
use std::{fs::OpenOptions, io::Write};

#[derive(Deserialize, Serialize)]
pub struct Project {
    pub repo: String,
    pub name: String,
    pub dir: String,
    pub links: Vec<String>,
    pub tags: Vec<String>,
}

impl Project {
    pub fn open_links(&self) -> Result<()> {
        for link in self.links.iter() {
            Command::new("xdg-open").arg(link).spawn()?;
        }
        Ok(())
    }
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

pub fn get_project_notes_file_path(taita_dir: &Path, project: &Project) -> Result<String> {
    let project_data_path = Path::new(&taita_dir).join(Path::new(&project.dir));
    if !project_data_path.exists() {
        std::fs::create_dir(project_data_path.clone())?;
    }

    let notes_path = project_data_path.join("notes.md");

    if !notes_path.exists() {
        OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(notes_path)
            .context("Failed to create notes file")?
            .write(format!("# `{}` - Notes\n\n## TODO:\n- [ ] A task", project.name).as_bytes())
            .context("Failed to write to notes file")?;
    }

    Ok(project_data_path
        .join("notes.md")
        .to_str()
        .context("Invalide unicode in the filepath")?
        .to_string())
}

pub fn get_default_projects_dir() -> Result<String> {
    Ok(
        Path::new(&std::env::var("HOME").context("Home dir not set")?)
            .join("Git")
            .to_str()
            .context("Failed to conver home dir to string")?
            .to_string(),
    )
}

pub fn get_default_taita_dir() -> Result<String> {
    Ok(
        Path::new(&std::env::var("HOME").context("Home dir not set")?)
            .join(".taita")
            .to_str()
            .context("Failed to conver home dir to string")?
            .to_string(),
    )
}
