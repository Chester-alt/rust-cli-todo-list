use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct Task{
    id: u32,
    description: String,
    completed: bool,
}

#[derive(Serialize, Deserialize, Debug)]
struct TodoList {
    tasks: Vec<Task>,
}

use clap::{App, Arg, SubCommand};
use std::fs;
use std::io::{self, Write};

fn main() {
    let matches = App::new("Todo List")
        .version("1.0")
        .author("Chester-Alt")
        .about("Manage Your tasks")
        .subcommand(SubCommand::with_name("add")
            .about("Add a new task")
            .arg(Arg::with_name("description")
                .help("The Description of the task")
                .required(true)
                .index(1)))
        .subcommand(SubCommand::with_name("complete")
                .about("Mark task as complete")
                .arg(Arg::with_name("id")
                    .help("The ID of the task to mark complete")
                    .required(true)
                    .index(1)))
        .subcommand(SubCommand::with_name("uncomplete")
                    .about("Mark task as incomplete")
                    .arg(Arg::with_name("id")
                        .help("The ID of the task to mark incomplete")
                        .required(true)
                        .index(1)))
        .subcommand(SubCommand::with_name("delete")
                    .about("Delete a task")
                    .arg(Arg::with_name("id")
                        .help("The ID of the task to do delete")
                        .required(true)
                        .index(1)))
        .subcommand(SubCommand::with_name("rename")
                        .about("Rename a task")
                        .arg(Arg::with_name("id")
                            .help("The ID of the task to rename")
                            .required(true)
                            .index(1))
                        .arg(Arg::with_name("description")
                            .help("The new description of the task")
                            .required(true)
                            .index(2)))
        .subcommand(SubCommand::with_name("list")
            .about("List all tasks"))
        .get_matches();

    let mut todo_list = load_tasks();

    if let Some(matches) = matches.subcommand_matches("add") {
        let description = matches.value_of("description").unwrap();
        add_task(&mut todo_list, description.to_string());
    } else if let Some(matches) = matches.subcommand_matches("complete") {
        let id: u32 = matches.value_of("id").unwrap().parse().unwrap();
        mark_task(&mut todo_list, id, true);
    } else if let Some(matches) = matches.subcommand_matches("uncomplete") {
        let id: u32 = matches.value_of("id").unwrap().parse().unwrap();
        mark_task(&mut todo_list, id, false);
    }else if let Some(matches) = matches.subcommand_matches("delete") {
        let id: u32 = matches.value_of("id").unwrap().parse().unwrap();
        delete_task(&mut todo_list, id);
    }else if let Some(matches) = matches.subcommand_matches("rename") {
        let id: u32 = matches.value_of("id").unwrap().parse().unwrap();
        let description = matches.value_of("description").unwrap();
        rename_task(&mut todo_list, id, description.to_string());
    }else if matches.subcommand_matches("list").is_some() {
        list_tasks(&todo_list);
    }

    save_tasks(&todo_list);

}

fn load_tasks() -> TodoList {
    if let Ok(data) = fs::read_to_string("tasks.json") {
        serde_json::from_str(&data).unwrap_or(TodoList { tasks: Vec::new() })
    } else {
        TodoList { tasks: Vec::new() }
    }
}

fn save_tasks(todo_list: &TodoList) {
    let data = serde_json::to_string_pretty(todo_list).unwrap();
    fs::write("tasks.json", data).unwrap();
}

fn add_task(todo_list: &mut TodoList, description: String) {
    let id = todo_list.tasks.len() as u32 + 1;
    let task = Task {id, description, completed: false };
    todo_list.tasks.push(task);
    print!("Task added successfully");
}   

fn mark_task(todo_list: &mut TodoList, id: u32, completed: bool) {
    if let Some(task) = todo_list.tasks.iter_mut().find(|task| task.id == id) {
        task.completed = completed;
        print!("Task marked as {}.", if completed {"complete"} else {"incomplete"});
    } else {
        println!("Task not found.");
    }
}

fn delete_task(todo_list: &mut TodoList, id: u32) {
    todo_list.tasks.retain(|task| task.id != id);
    print!("Task delete.");
}

fn rename_task(todo_list: &mut TodoList, id: u32, description: String) {
    if let Some(task) = todo_list.tasks.iter_mut().find(|task| task.id == id) {
        task.description = description;
        println!("Task renamed.");
    } else {
            println!("Task not found");
    }
}

fn list_tasks(todo_list: &TodoList) {
    if todo_list.tasks.is_empty() {
        println!("No tasks available");
    } else {
        for task in &todo_list.tasks {
            println!("ID: {}, Description: {}, Completed: {}", task.id, task.description, task.completed);
        }
    }
}
