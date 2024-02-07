use serde::{Deserialize, Serialize};
use std::{
    fs::{self, OpenOptions},
    io::{self, Write},
    path::Path,
};

use clap::{Parser, Subcommand};

#[derive(Deserialize, Serialize)]
struct Project {
    repo: String,
    name: String,
    folder: String,
    tags: Vec<String>,
}

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    #[arg(short, long)]
    projects: Option<String>,

    #[arg(short, long)]
    directory: Option<String>,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Ls,
    Open {
        name: String,
    },
    Add {
        repo: String,

        #[arg(short, long)]
        name: Option<String>,

        #[arg(short, long)]
        folder: Option<String>,

        #[arg(short, long)]
        tags: Vec<String>,
    },
    Edit {
        old_name: String,

        #[arg(short, long)]
        name: Option<String>,

        #[arg(short, long)]
        folder: Option<String>,

        #[arg(short, long)]
        repo: Option<String>,

        #[arg(short, long)]
        tags: Vec<String>,

        #[arg(short, long)]
        untags: Vec<String>,
    },
    Rm {
        name: String,
    },
}

#[derive(Deserialize, Serialize)]
struct Config {}

fn read_projects(projects_file_path: &Path) -> Vec<Project> {
    if projects_file_path.exists() {
        serde_json::from_str(
            &fs::read(projects_file_path)
                .expect("Failed to read projects data file")
                .iter()
                .map(|i| *i as char)
                .collect::<String>(),
        )
        .expect("Invalid data in project data file")
    } else {
        vec![]
    }
}

fn repo_to_url(repo: &str) -> String {
    if repo.contains("://") {
        repo.to_string()
    } else {
        format!("git@github.com:{}", repo)
    }
}

fn save_projects(projects: &Vec<Project>, projects_file_path: &Path) -> Result<(), io::Error> {
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(projects_file_path)?;

    file.write_all(
        &serde_json::to_string_pretty(&projects)
            .unwrap()
            .chars()
            .map(|c| c as u8)
            .collect::<Vec<u8>>(),
    )?;

    Ok(())
}

fn main() {
    let args = Args::parse();

    let taita_dir = match args.directory {
        None => {
            let default_taita_dir = "/home/zachiahsawyer/.taita";

            if !Path::new(default_taita_dir).exists() {
                std::fs::create_dir(default_taita_dir)
                    .expect("Failed to create taita default directory")
            }

            default_taita_dir.into()
        }
        Some(d) => d,
    };
    let projects_dir = args.projects.unwrap_or("/home/zachiahsawyer/Git".into());
    let projects_file_path = Path::new(&taita_dir).join("projects.json");

    match args.command {
        Commands::Ls => {
            let projects = read_projects(&projects_file_path);
            if projects.len() == 0 {
                println!("You don't have any projects. Learn how with:\n$ taita help add");
            }
            for project in projects.iter() {
                let name = &project.name;
                let folder = &project.folder;
                let repository = &project.repo;
                let tags = project.tags.join(", ");
                println!(
                    "{name}
Folder: {folder}
Repository: {repository}
Tags: {tags}",
                );
            }
        }
        Commands::Open { name } => {
            let projects = read_projects(&projects_file_path);

            let project = projects
                .iter()
                .find(|p| p.name == name)
                .expect("No project with that name");

            let project_path = Path::new(&projects_dir).join(Path::new(&project.folder));
            println!("{:?}", project_path);
            if !project_path.exists() {
                let result = std::process::Command::new("git")
                    .arg("clone")
                    .arg(repo_to_url(&project.repo))
                    .arg(project.folder.clone())
                    .current_dir(projects_dir)
                    .output()
                    .unwrap();

                if !result.status.success() {
                    panic!(
                        "Failed to clone repo:\n{}",
                        std::str::from_utf8(&result.stderr).unwrap()
                    );
                }
            }

            std::process::Command::new("alacritty")
                .args(vec![
                    "--working-directory",
                    project_path.to_str().unwrap(),
                    "--command",
                    "tmux",
                ])
                .spawn()
                .unwrap();
        }
        Commands::Add {
            name,
            repo,
            folder,
            tags,
        } => {
            let mut projects = read_projects(&projects_file_path);
            projects.push(Project {
                repo: repo.clone(),
                folder: folder.unwrap_or(repo.split('/').last().unwrap().to_string()),
                name: name.unwrap_or(repo.split('/').last().unwrap().to_string()),
                tags,
            });
            save_projects(&projects, &projects_file_path).expect("Failed to save projects");
        }
        Commands::Rm { name } => {
            let mut projects = read_projects(&projects_file_path);
            projects.remove(
                projects
                    .iter()
                    .position(|p| p.name == name)
                    .expect("Failed to find project of that name"),
            );
            save_projects(&projects, &projects_file_path).expect("Failed to save projects");
        }
        Commands::Edit {
            old_name,
            name,
            folder,
            repo,
            tags,
            untags,
        } => {
            let mut projects = read_projects(&projects_file_path);
            let index = projects
                .iter()
                .position(|p| p.name == old_name)
                .expect("Failed to find project of that name");

            projects[index].name = name.unwrap_or(projects[index].name.clone());
            projects[index].folder = folder.unwrap_or(projects[index].folder.clone());
            projects[index].repo = repo.unwrap_or(projects[index].repo.clone());
            projects[index].tags = projects[index]
                .tags
                .iter()
                .chain(tags.iter())
                .filter(|t| !untags.contains(t))
                .map(|t| t.to_string())
                .collect();

            save_projects(&projects, &projects_file_path).expect("Failed to save projects");
        }
    };
}
