use super::objects::{Task, TaskState};
use chrono::{DateTime, Duration, Local, Utc};
use colored::Colorize;

const TASK_CHARS: [char; 4] = ['b', 'd', 'i', 'D'];
const DATE_DISPLAY_FORMAT: &str = "%F %R";

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

    new_string = new_string.replace("%i", &format!("#{}", id).bright_black().to_string());
    match to_format.due {
        Some(d) => new_string.replace("%D", &format_due_date(d)),
        None => new_string.replace("-> %D", ""),
    }
}

pub fn format_due_date(due: DateTime<Utc>) -> String {
    let now = Utc::now();
    let mut negative = false;
    let diff = if due - now < Duration::days(0) {
        negative = true;
        now - due
    } else {
        due - now
    };

    let mut new_string = String::new();
    if diff < Duration::minutes(1) {
        let second_str = if diff.num_seconds() == 1 {
            "second"
        } else {
            "seconds"
        };
        new_string.push_str(
            &format!("{} {}", diff.num_seconds(), second_str)
                .red()
                .to_string(),
        );
    } else if diff < Duration::hours(1) {
        let minute_str = if diff.num_minutes() == 1 {
            "minute"
        } else {
            "minutes"
        };
        new_string.push_str(
            &format!("{} {}", diff.num_minutes(), minute_str)
                .red()
                .to_string(),
        );
    } else if diff < Duration::days(1) {
        let hour_str = if diff.num_hours() == 1 {
            "hour"
        } else {
            "hours"
        };
        new_string.push_str(
            &format!("{} {}", diff.num_hours(), hour_str)
                .red()
                .to_string(),
        );
    } else if diff < Duration::weeks(4) {
        if diff.num_days() == 1 {
            new_string.push_str(&format!("{} day", diff.num_days()).red().to_string());
        } else if diff.num_days() > 5 {
            new_string.push_str(&format!("{} days", diff.num_days()).bright_red().to_string());
        } else {
            new_string.push_str(&format!("{} days", diff.num_days()).bright_red().to_string());
        }
    } else if diff < Duration::weeks(52) {
        let month_str = if diff.num_weeks() < 6 {
            "month"
        } else {
            "months"
        };
        new_string.push_str(&format!("{} {}", diff.num_weeks() / 4, month_str));
    } else {
        let year_str = if diff.num_weeks() < 104 {
            "year"
        } else {
            "years"
        };
        new_string.push_str(&format!("{} {}", diff.num_weeks() / 52, year_str));
    }

    if negative {
        new_string = new_string.red().bold().to_string();
    }

    new_string
}

pub fn format_task_spotlight(task: &Task, id: usize) -> String {
    let mut new_string = String::from("\n");

    match task.state {
        TaskState::Todo => new_string.push_str("[ ] "),
        TaskState::Done => new_string.push_str("[X] "),
    }

    new_string.push_str(&task.description.bold().to_string());
    new_string.push('\n');
    new_string.push_str(&String::from("Due: ").bright_black().to_string());
    match task.due {
        Some(d) => {
            new_string.push_str(&format_due_date(d));
            new_string.push_str(
                &format!(" ({})", d.with_timezone(&Local).format(DATE_DISPLAY_FORMAT))
                    .bright_black()
                    .to_string(),
            );
        }
        None => new_string.push_str(&String::from("None").bright_black().to_string()),
    }
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
