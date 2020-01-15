use super::objects::{Task, TaskState};
use colored::Colorize;

const TASK_CHARS: [char; 3] = ['b', 'd', 'i'];

pub fn format_task(format_string: &String, to_format: &Task, id: usize) -> String {
    let mut new_string = String::new();
    new_string.push_str(format_string);

    let mut greyout = false;
    new_string = new_string.replace(
        "%b",
        match to_format.state {
            TaskState::Todo => "[ ]",
            TaskState::Done => {
                greyout = true;
                "[X]"
            }
        },
    );

    if greyout {
        new_string = new_string
            .replace("%d", &to_format.description[..])
            .bright_black()
            .to_string();
    } else {
        new_string = new_string.replace("%d", &to_format.description[..]);
    }

    new_string.replace("%i", &format!("#{}", id).bright_black().to_string())
}

pub fn format_task_spotlight(task: &Task, id: usize) -> String {
    let mut new_string = String::from("\n");

    match task.state {
        TaskState::Todo => new_string.push_str("[ ] "),
        TaskState::Done => new_string.push_str("[X] "),
    }

    new_string.push_str(&task.description.bold().to_string());
    new_string.push('\n');
    new_string.push_str(
        &format!(
            "category: {}\n",
            task.category.as_ref().unwrap_or(&String::from("None"))
        )
        .bright_black()
        .to_string(),
    );
    new_string.push_str(&format!("id: #{}\n", id).bright_black().to_string());
    new_string.push('\n');
    new_string.push_str(
        &task
            .information
            .as_ref()
            .unwrap_or(&String::from("No information.").bright_black().to_string()),
    );
    new_string.push('\n');

    new_string
}

pub fn format_category(category: &String, ids: &Vec<usize>) -> String {
    let mut new_string = String::new();

    new_string.push_str(&category.bold().to_string());
    new_string.push_str(": ");
    new_string.push_str(&format!("{} tasks", ids.len()).bright_black().to_string());
    new_string
}

pub fn format_context(context: &String, is_current: bool) -> String {
    let mut new_string = String::new();

    if is_current {
        new_string.push_str(&format!("~{}~", &context.italic().to_string()));
    } else {
        new_string.push_str(&context.to_string());
    }

    new_string
}

pub fn format_nothing() -> String {
    String::from("No tasks!").bright_black().to_string()
}

pub fn valid_task_format(format: &String) -> bool {
    valid(format, TASK_CHARS.to_vec())
}

fn valid(format: &String, valid_letters: Vec<char>) -> bool {
    let mut follows_escape = false;
    for character in format.chars() {
        if follows_escape {
            if !valid_letters.contains(&character) {
                return false;
            }
            follows_escape = false;
        }
        if character == '%' {
            follows_escape = true
        }
    }

    true
}

#[cfg(test)]
mod tests {
    #[test]
    fn format_string_validation() {
        use super::*;

        assert!(valid_task_format(&String::from("%b %d")));
        assert!(!valid_task_format(&String::from("%a %z")));
    }
}
