use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::{
    fs::{self, File, OpenOptions},
    io::Write,
    path::Path,
    process::ExitStatus,
};

use clap::{Parser, Subcommand};

#[derive(Deserialize, Serialize)]
struct Project {
    repo: String,
    name: String,
    folder: String,
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
            for project in projects.iter() {
                println!(
                    "{}\nFolder: {}\nURL: {}\n",
                    project.name, project.folder, project.repo
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
        Commands::Add { name, repo, folder } => {
            let mut projects = read_projects(&projects_file_path);
            projects.push(Project {
                repo: repo.clone(),
                folder: folder.unwrap_or(repo.split('/').last().unwrap().to_string()),
                name: name.unwrap_or(repo.split('/').last().unwrap().to_string()),
            });

            let mut file = OpenOptions::new()
                .write(true)
                .create(true)
                .open(projects_file_path)
                .unwrap();

            file.write_all(
                &serde_json::to_string_pretty(&projects)
                    .unwrap()
                    .chars()
                    .map(|c| c as u8)
                    .collect::<Vec<u8>>(),
            )
            .unwrap();
        }
    };
}
