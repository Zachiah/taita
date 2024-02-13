use crate::projects_file::{
    get_project_notes_file_path, get_project_position, read_projects, save_projects, Project,
};
use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use std::{os::unix::process::CommandExt, path::Path};

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
    OpenInPlace {
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
    Notes {
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
    let mut projects = read_projects(&projects_file_path)?;

    match args.command {
        Commands::Ls { picker } => {
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

            let current_executable =
                std::env::current_exe().context("Failed to get current executable path")?;

            let current_exe = current_executable
                .to_str()
                .context("Failed to convert current executable path to string")?;

            if std::env::var("TERM") == Ok("alacritty".to_string()) {
                std::process::Command::new(current_exe)
                    .arg("open-in-place")
                    .arg(name)
                    .exec();
            } else {
                let script = format!(
                    r#"
                        alacritty --command bash -c "{current_exe} open-in-place {name}"
                    "#,
                );
                std::process::Command::new("sh")
                    .args(vec!["-c", script.as_str()])
                    .spawn()
                    .context("Failed to launch alacritty")?;
            }
        }
        Commands::OpenInPlace { name } => {
            let projects = read_projects(&projects_file_path).expect("Failed to read project file");

            let project = projects
                .iter()
                .find(|p| p.name == name)
                .expect("No project with that name");

            let project_path = Path::new(&projects_dir).join(Path::new(&project.folder));
            let notes_file_path = get_project_notes_file_path(Path::new(&taita_dir), project)?;
            let project_path = project_path.to_str().unwrap();

            let script = format!(
                r#"
                    tmux has-session -t taita-{name} 2>/dev/null

                    if [ $? != 0 ]; then
                        cd {project_path}
                        tmux new-session -d -s taita-{name} "nvim {notes_file_path}"
                        tmux split-window -l 4 -t taita-{name}
                        tmux select-pane -t taita-{name}:0.0
                    fi
                "#,
            );
            std::process::Command::new("sh")
                .args(vec!["-c", script.as_str()])
                .output()
                .context("Failed to open project")?;

            std::process::Command::new("tmux")
                .arg("a")
                .arg("-t")
                .arg("taita-".to_string() + &name)
                .exec();
        }
        Commands::Add {
            name,
            repo,
            folder,
            tags,
        } => {
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
            projects.remove(get_project_position(&projects, name)?);
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
        Commands::Notes { name } => {
            let index = get_project_position(&projects, name)?;
            let project = &projects[index];
            println!(
                "{}",
                get_project_notes_file_path(Path::new(&taita_dir), &project)?
            );
        }
    };

    Ok(())
}
