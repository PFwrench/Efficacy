use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::PathBuf;

use super::{errors::EfficacyError, objects, settings::Settings};

#[derive(Debug)]
pub struct State<'a> {
    settings: &'a Settings,
    task_file_path: PathBuf,
    pub task_objects: Vec<objects::Task>,
    pub category_map: HashMap<String, Vec<usize>>,
}

impl<'a> State<'a> {
    /// Checks for existence of or creates directories and files used in maintaining state.
    pub fn new(settings: &'a Settings) -> Result<Self, EfficacyError> {
        let mut continue_creating = false;

        let data_dir = PathBuf::from(&settings.data_file_path);
        if !data_dir.exists() {
            std::fs::create_dir(data_dir)?;
            continue_creating = true;
        }

        let task_file_path =
            PathBuf::from(&settings.data_file_path).join(PathBuf::from("task.json"));
        if continue_creating || !task_file_path.exists() {
            File::create(&task_file_path)?;
            continue_creating = true;
        }

        let mut new_state = State {
            settings,
            task_file_path,
            task_objects: Vec::new(),
            category_map: HashMap::new(),
        };

        if !continue_creating {
            match new_state.load() {
                Ok(_) => (),
                Err(e) => {
                    println!("Issue when loading state for the first time...");
                    return Err(e);
                }
            };
        }

        Ok(new_state)
    }

    pub fn save(&self) -> Result<(), EfficacyError> {
        let tasks_serialized = serde_json::to_string(&self.task_objects).unwrap();

        OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(&self.task_file_path)?
            .write(tasks_serialized.as_bytes())?;

        Ok(())
    }

    pub fn load(&mut self) -> Result<(), EfficacyError> {
        let mut tasks_string = String::new();

        OpenOptions::new()
            .read(true)
            .open(&self.task_file_path)?
            .read_to_string(&mut tasks_string)?;

        let tasks: Vec<objects::Task> = match serde_json::from_str(&tasks_string[..]) {
            Ok(o) => o,
            Err(_) => Vec::new(),
        };

        for (id, task) in tasks.iter().enumerate() {
            self.add_to_category_map(task, id);
        }

        self.task_objects = tasks;

        Ok(())
    }

    pub fn add_to_category_map(&mut self, task: &objects::Task, id: usize) {
        match &task.category {
            Some(c) => match self.category_map.get_mut(c) {
                Some(v) => v.push(id),
                None => {
                    self.category_map.insert(c.to_string(), vec![id]);
                }
            },
            None => match self.category_map.get_mut("No category") {
                Some(v) => v.push(id),
                None => {
                    self.category_map
                        .insert(String::from("No category"), vec![id]);
                }
            },
        }
    }

    pub fn rebuild_category_map(&mut self) {
        self.category_map.clear();

        for (id, task) in self.task_objects.iter().enumerate() {
            match &task.category {
                Some(c) => match self.category_map.get_mut(c) {
                    Some(v) => v.push(id),
                    None => {
                        self.category_map.insert(c.to_string(), vec![id]);
                    }
                },
                None => match self.category_map.get_mut("No category") {
                    Some(v) => v.push(id),
                    None => {
                        self.category_map
                            .insert(String::from("No category"), vec![id]);
                    }
                },
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::objects::{Task, TaskState};
    use super::State;

    fn generate_test_tasks() -> Vec<Task> {
        let task_1 = Task {
            description: String::from("Add classes to calendar"),
            state: TaskState::Done,
            category: Option::Some(String::from("School")),
        };
        let task_2 = Task {
            description: String::from("Study for exam"),
            state: TaskState::Todo,
            category: Option::Some(String::from("School")),
        };
        let task_3 = Task {
            description: String::from("Get haircut"),
            state: TaskState::Todo,
            category: Option::Some(String::from("Personal")),
        };
        let task_4 = Task {
            description: String::from("Workout"),
            state: TaskState::Todo,
            category: Option::None,
        };

        vec![task_1, task_2, task_3, task_4]
    }

    #[test]
    fn save_and_load() {
        use super::super::settings::Settings;

        let config = Settings::new().unwrap();
        let mut state = State::new(&config).unwrap();

        let tasks_before = generate_test_tasks();
        state.task_objects = tasks_before;

        state.save().unwrap();
        state.load().unwrap();
    }
}
