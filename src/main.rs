use std::{
    env, fs,
    io::{self, Read, Write},
};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    match args.get(1) {
        Some(comm) if comm == "add" => {
            add(args)?;
        }
        Some(comm) if comm == "remove" => {
            remove(args)?;
        }
        Some(comm) if comm == "clear" => {
            clear()?;
        }
        Some(comm) if comm == "list" => {
            list()?;
        }
        Some(comm) => {
            println!("Unknown command: {comm}");
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
        eprintln!("No task name given");
        return Ok(());
    };

    let home = env::var("HOME").expect("Failed to get home-directory");
    let content = match fs::read_to_string(format!("{home}/.todo/list.todo")) {
        Ok(content) => content,
        Err(_) => "".to_string(),
    };

    let new_content = if content.is_empty() {
        format!("{task}")
    } else {
        format!("{content}\n{task}")
    };

    fs::create_dir_all(format!("{home}/.todo"))?;
    let mut file = fs::File::create(format!("{home}/.todo/list.todo"))?;
    file.write_all(new_content.as_bytes())?;

    Ok(())
}

fn remove(args: Vec<String>) -> io::Result<()> {
    let Some(task_index) = args.get(2) else {
        eprintln!("No index given");
        return Ok(());
    };
    let Ok(task_index) = task_index.parse::<usize>() else {
        eprintln!("Invalid index format");
        return Ok(());
    };
    let task_index = task_index - 1;

    let home = env::var("HOME").expect("Failed to get home-directory");
    let content = match fs::read_to_string(format!("{home}/.todo/list.todo")) {
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
    let mut file = fs::File::create(format!("{home}/.todo/list.todo"))?;
    file.write_all(new_content.as_bytes())?;

    Ok(())
}

fn clear() -> io::Result<()> {
    let home = env::var("HOME").expect("Failed to get home-directory");
    fs::remove_file(format!("{home}/.todo/list.todo"))?;
    Ok(())
}

fn list() -> io::Result<()> {
    let no_tasks_text = "No tasks";
    let home = env::var("HOME").expect("Failed to get home-directory");
    let Ok(mut file) = fs::File::open(format!("{home}/.todo/list.todo")) else {
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
