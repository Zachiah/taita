use crate::projects_file::{read_projects, save_projects, Project, get_project_position};
use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use std::path::Path;

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
    Ls {
        #[arg(short, long, default_value_t = false)]
        picker: bool,
    },
    Open {
        name: String,

        #[arg(short, long, default_value_t = false)]
        picker: bool,
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

fn repo_to_url(repo: &str) -> String {
    if repo.contains("://") {
        repo.to_string()
    } else {
        format!("git@github.com:{}", repo)
    }
}

pub fn cli() -> Result<()> {
    let args = Args::parse();

    let taita_dir = match args.directory {
        None => {
            let default_taita_dir = "/home/zachiahsawyer/.taita";

            if !Path::new(default_taita_dir).exists() {
                std::fs::create_dir(default_taita_dir)
                    .context("Failed to create taita default directory")?;
            }

            default_taita_dir.into()
        }
        Some(d) => d,
    };
    let projects_dir = args.projects.unwrap_or("/home/zachiahsawyer/Git".into());
    let projects_file_path = Path::new(&taita_dir).join("projects.json");

    match args.command {
        Commands::Ls { picker } => {
            let projects = read_projects(&projects_file_path)?;
            if projects.len() == 0 {
                println!("You don't have any projects. Learn how with:\n$ taita help add");
            }
            for project in projects.iter() {
                let name = &project.name;
                let folder = &project.folder;
                let repository = &project.repo;
                let tags = project.tags.join(", ");
                let tags_picker = project
                    .tags
                    .iter()
                    .map(|t| "#".to_string() + t)
                    .collect::<Vec<_>>()
                    .join(", ");
                if picker {
                    println!("{name} - {tags_picker}");
                } else {
                    println!(
                        "{name}
Folder: {folder}
Repository: {repository}
Tags: {tags}
",
                    );
                }
            }
        }
        Commands::Open { name, picker } => {
            let name = if picker {
                name.split(" - ")
                    .next()
                    .context("Invalid name from picker")?
                    .to_string()
            } else {
                name
            };

            let projects = read_projects(&projects_file_path).expect("Failed to read project file");

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
            let mut projects =
                read_projects(&projects_file_path)?;

            // Unwrap used below due to the data being validated already from read_projects
            projects.push(Project {
                repo: repo.clone(),
                folder: folder.unwrap_or(repo.split('/').last().unwrap().to_string()),
                name: name.unwrap_or(repo.split('/').last().unwrap().to_string()),
                tags,
            });
            save_projects(&projects, &projects_file_path)?;
        }
        Commands::Rm { name } => {
            let mut projects = read_projects(&projects_file_path)?;
            projects.remove(
                get_project_position(&projects, name)?
            );
            save_projects(&projects, &projects_file_path)?;
        }
        Commands::Edit {
            old_name,
            name,
            folder,
            repo,
            tags,
            untags,
        } => {
            let mut projects = read_projects(&projects_file_path)?;
            let index = get_project_position(&projects, old_name)?;

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

            save_projects(&projects, &projects_file_path)?;
        }
    };

    Ok(())
}
