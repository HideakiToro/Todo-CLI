use std::{
    env, fs,
    io::{self, Read, Write},
};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    match args.get(1) {
        Some(command) if command == "add" => {
            add(args)?;
        }
        Some(command) if command == "remove" => {
            remove(args)?;
        }
        Some(command) if command == "clear" => {
            clear(args)?;
        }
        Some(command) if command == "list" => {
            list(args)?;
        }
        Some(command) if command == "projects" => {
            projects(args)?;
        }
        Some(command) => {
            println!("Unknown command: {command}");
        }
        None => {
            println!(
                "Usage: todo {{command}} {{task-name}} {{modifiers}}\n\nAvailable commands:\n\nadd\nremove\nlist\nclear"
            );
        }
    }

    Ok(())
}

fn add(args: Vec<String>) -> io::Result<()> {
    let Some(task) = args.get(2) else {
        eprintln!("No task-name given");
        return Ok(());
    };

    if task == "--help" || task == "-h" {
        println!("Usage: todo add {{task-name}} -p {{project-name}}");
        return Ok(());
    }

    let mut project = "default".to_string();

    match args.get(3) {
        Some(modifier) if modifier == "-p" => {
            let Some(new_project) = args.get(4) else {
                let task = if task.contains(" ") {
                    format!("\"{task}\"")
                } else {
                    task.clone()
                };
                println!("Usage: todo add {task} -p {{project-name}}");
                return Ok(());
            };

            project = new_project.replace(' ', "_");
        }
        _ => {}
    }

    let home = env::var("HOME").expect("Failed to get home-directory");
    let content = match fs::read_to_string(format!("{home}/.todo/projects/{project}.todo")) {
        Ok(content) => content,
        Err(_) => "".to_string(),
    };

    let new_content = if content.is_empty() {
        format!("{task}")
    } else {
        format!("{content}\n{task}")
    };

    fs::create_dir_all(format!("{home}/.todo/projects"))?;
    let mut file = fs::File::create(format!("{home}/.todo/projects/{project}.todo"))?;
    file.write_all(new_content.as_bytes())?;

    Ok(())
}

fn remove(args: Vec<String>) -> io::Result<()> {
    let Some(task_index) = args.get(2) else {
        eprintln!("No index given");
        return Ok(());
    };

    if task_index == "--help" || task_index == "-h" {
        println!("Usage: todo add {{task-name}} -p {{project-name}}");
        return Ok(());
    }

    let mut project = "default".to_string();

    match args.get(3) {
        Some(modifier) if modifier == "-p" => {
            let Some(new_project) = args.get(4) else {
                let task_index = if task_index.contains(" ") {
                    format!("\"{task_index}\"")
                } else {
                    task_index.clone()
                };
                let Ok(task_index) = task_index.parse::<usize>() else {
                    eprintln!("Invalid index format");
                    return Ok(());
                };
                println!("Usage: todo remove {task_index} -p {{project-name}}");
                return Ok(());
            };

            project = new_project.replace(' ', "_");
        }
        _ => {}
    }

    let Ok(task_index) = task_index.parse::<usize>() else {
        eprintln!("Invalid index format");
        return Ok(());
    };
    let task_index = task_index - 1;

    let home = env::var("HOME").expect("Failed to get home-directory");
    let content = match fs::read_to_string(format!("{home}/.todo/projects/{project}.todo")) {
        Ok(content) => content,
        Err(_) => "".to_string(),
    };

    if content.is_empty() {
        println!("No tasks");
        return Ok(());
    }

    let mut lines: Vec<&str> = content
        .split("\n")
        .filter(|line| !line.is_empty())
        .collect();

    lines.remove(task_index);

    let new_content = lines.join("\n");
    let mut file = fs::File::create(format!("{home}/.todo/projects/{project}.todo"))?;
    file.write_all(new_content.as_bytes())?;

    if lines.is_empty() {
        clear(vec!["todo".into(), "clear".into(), "-p".into(), project])?;
    }

    Ok(())
}

fn clear(args: Vec<String>) -> io::Result<()> {
    let mut project = "default".to_string();

    match args.get(2) {
        Some(modifier) if modifier == "-p" => {
            let Some(new_project) = args.get(3) else {
                println!("Usage: todo clear -p {{project-name}}");
                return Ok(());
            };

            project = new_project.replace(' ', "_");
        }
        Some(modifier) if modifier == "--help" || modifier == "-h" => {
            println!("Usage: todo clear -p {{project-name}}");
            return Ok(());
        }
        _ => {}
    }

    let home = env::var("HOME").expect("Failed to get home-directory");
    fs::remove_file(format!("{home}/.todo/projects/{project}.todo"))?;
    Ok(())
}

fn list(args: Vec<String>) -> io::Result<()> {
    let no_tasks_text = "No tasks";

    let mut project = "default".to_string();

    match args.get(2) {
        Some(modifier) if modifier == "-p" => {
            let Some(new_project) = args.get(3) else {
                println!("Usage: todo list -p {{project-name}}");
                return Ok(());
            };

            project = new_project.replace(' ', "_");
        }
        Some(modifier) if modifier == "--help" || modifier == "-h" => {
            println!("Usage: todo list -p {{project-name}}");
            return Ok(());
        }
        _ => {}
    }

    let home = env::var("HOME").expect("Failed to get home-directory");
    let Ok(mut file) = fs::File::open(format!("{home}/.todo/projects/{project}.todo")) else {
        println!("{no_tasks_text}");
        return Ok(());
    };

    let mut content = String::new();
    file.read_to_string(&mut content)?;

    let lines: Vec<&str> = content.split("\n").collect();
    if lines.is_empty() {
        println!("{no_tasks_text}");
        return Ok(());
    }

    println!("Tasks:\n");
    for (i, line) in lines.iter().enumerate() {
        println!("{}. {line}", i + 1);
    }

    Ok(())
}

fn projects(args: Vec<String>) -> io::Result<()> {
    match args.get(2) {
        Some(command) if command == "list" => {
            projects_list(args)?;
        }
        Some(command) if command == "clear" => {
            projects_clear(args)?;
        }
        Some(command) if command == "remove" => {
            projects_remove(args)?;
        }
        _ => {
            println!("Usage: todo projects {{command}} {{project-name}} {{modifiers}}");
        }
    }
    return Ok(());
}

fn projects_list(_args: Vec<String>) -> io::Result<()> {
    let no_projects_text = "No projects";

    let home = env::var("HOME").expect("Failed to get home-directory");
    let Ok(mut dirs) = fs::read_dir(format!("{home}/.todo/projects")) else {
        println!("{no_projects_text}");
        return Ok(());
    };

    let mut projects = Vec::new();
    while let Some(dir) = dirs.next()
        && let Ok(dir) = dir
    {
        let file_name = dir.file_name();
        let Some(project_name) = file_name.to_str() else {
            eprintln!("Failed to read project-name");
            return Ok(());
        };
        let mut project = project_name.to_string();
        if project.ends_with(".todo") && project != "default.todo".to_string() {
            for _ in 0..5 {
                project.pop();
            }
            projects.push(project);
        }
    }

    if projects.is_empty() {
        println!("{no_projects_text}");
        return Ok(());
    }

    println!("Projects:\n");
    for project in projects {
        println!("{}", project);
    }
    return Ok(());
}

fn projects_clear(_args: Vec<String>) -> io::Result<()> {
    let home = env::var("HOME").expect("Failed to get home-directory");

    fs::remove_dir_all(format!("{home}/.todo/projects")).expect("Failed to clear projects");
    return Ok(());
}

fn projects_remove(mut args: Vec<String>) -> io::Result<()> {
    // Remove "todo projects remove" so that project-name is at index 0
    for _ in 0..3 {
        args.remove(0);
    }

    let mut new_args = vec!["todo".into(), "clear".into(), "-p".into()];
    for arg in args {
        new_args.push(arg);
    }

    clear(new_args)?;
    return Ok(());
}
