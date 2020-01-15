use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::PathBuf;

use super::{errors::EfficacyError, objects, settings::Settings};

#[derive(Debug)]
pub struct State<'a> {
    settings: &'a Settings,
    context_file_path: PathBuf,
    pub current_context: objects::Context,
    pub task_file_paths: HashMap<String, PathBuf>,
    pub task_objects: Vec<objects::Task>,
    pub category_map: HashMap<String, Vec<usize>>,
}

// Core State functionality
impl<'a> State<'a> {
    /// Checks for existence of or creates directories and files used in maintaining state.
    pub fn new(settings: &'a Settings) -> Result<Self, EfficacyError> {
        let mut continue_creating = false;

        let data_dir = PathBuf::from(&settings.data_file_path);
        if !data_dir.exists() {
            std::fs::create_dir(data_dir)?;
            continue_creating = true;
        }

        let default_context = objects::Context {
            context_name: String::from("default"),
        };

        let context_file_path =
            PathBuf::from(&settings.data_file_path).join(PathBuf::from("context.json"));
        if continue_creating || !context_file_path.exists() {
            let context_serialized = serde_json::to_string(&default_context).unwrap();

            OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(&context_file_path)?
                .write(context_serialized.as_bytes())?;

            continue_creating = true;
        }

        let default_task_file_path =
            PathBuf::from(&settings.data_file_path).join(PathBuf::from("default.json"));
        if continue_creating || !default_task_file_path.exists() {
            File::create(&default_task_file_path)?;
            continue_creating = true;
        }

        let mut task_file_paths = HashMap::new();

        for entry in std::fs::read_dir(&settings.data_file_path)? {
            let entry = entry?;
            let path = entry.path();
            if !path.as_os_str().eq(context_file_path.as_os_str())
                && !path.as_os_str().eq(default_task_file_path.as_os_str())
            {
                let path_key = String::from(path.file_stem().unwrap().to_str().unwrap());
                task_file_paths.insert(path_key, path);
            }
        }

        task_file_paths.insert(String::from("default"), default_task_file_path);

        let mut new_state = State {
            settings,
            context_file_path,
            current_context: default_context,
            task_file_paths,
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

    pub fn save(&mut self) -> Result<(), EfficacyError> {
        let tasks_serialized = serde_json::to_string(&self.task_objects).unwrap();

        let fp_result = &self.task_file_paths.get(&self.current_context.context_name);

        let file_path = match fp_result {
            Some(pb) => pb,
            None => return Err(EfficacyError::MalformedContextError),
        };

        OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(file_path)?
            .write(tasks_serialized.as_bytes())?;

        self.save_context()
    }

    pub fn load(&mut self) -> Result<(), EfficacyError> {
        let mut tasks_string = String::new();

        self.load_context()?;

        let fp_result = &self.task_file_paths.get(&self.current_context.context_name);
        let file_path = match fp_result {
            Some(pb) => pb,
            None => return Err(EfficacyError::MalformedContextError),
        };

        OpenOptions::new()
            .read(true)
            .open(file_path)?
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
}

// Category map operations
impl<'a> State<'a> {
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

// Context operations
impl<'a> State<'a> {
    pub fn save_context(&self) -> Result<(), EfficacyError> {
        let context_serialized = serde_json::to_string(&self.current_context).unwrap();

        OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(&self.context_file_path)?
            .write(context_serialized.as_bytes())?;

        Ok(())
    }

    pub fn load_context(&mut self) -> Result<(), EfficacyError> {
        let mut ctx_string = String::new();

        OpenOptions::new()
            .read(true)
            .open(&self.context_file_path)?
            .read_to_string(&mut ctx_string)?;

        self.current_context = match serde_json::from_str(&ctx_string[..]) {
            Ok(o) => o,
            Err(_) => objects::Context {
                context_name: String::from("default"),
            },
        };

        Ok(())
    }

    pub fn new_context(&mut self, context_name: &String) -> Result<(), EfficacyError> {
        if context_name.eq(&String::from("default")) {
            println!(
                "Cannot make a new context with the name 'default', that context is reserved."
            );
            return Err(EfficacyError::BadContextNameError);
        }

        let trimmed_context_name = String::from(context_name.trim());
        if trimmed_context_name.contains(" ") {
            return Err(EfficacyError::BadContextNameError);
        }

        let new_context_task_file_path = PathBuf::from(&self.settings.data_file_path).join(
            PathBuf::from(format!("{}{}", &trimmed_context_name, ".json")),
        );

        std::fs::File::create(&new_context_task_file_path)?;

        self.task_file_paths
            .insert(trimmed_context_name.clone(), new_context_task_file_path);

        self.change_context(&trimmed_context_name)
    }

    pub fn change_context(&mut self, context_name: &String) -> Result<(), EfficacyError> {
        if !self.context_exists(context_name) {
            println!("Context '{}' does not exist", context_name);
            return Err(EfficacyError::BadContextNameError);
        }

        self.save()?;

        if !self.task_file_paths.contains_key(context_name) {
            return Err(EfficacyError::BadContextNameError);
        }

        self.current_context.context_name = context_name.clone();
        self.save_context()?;
        self.load()?;
        self.rebuild_category_map();

        Ok(())
    }

    pub fn delete_context(&mut self, context_name: &String) -> Result<(), EfficacyError> {
        if context_name.eq("default") {
            println!("Cannot delete the default context.");
            return Ok(());
        }

        if context_name.eq(&self.current_context.context_name) {
            println!(
                "Cannot delete the current context. Switch to another context before deleting."
            );
            return Ok(());
        }

        let file_to_delete = match self.task_file_paths.get(context_name) {
            Some(p) => p,
            None => {
                println!("Context given does not exist.");
                return Ok(());
            }
        };

        std::fs::remove_file(file_to_delete)?;

        Ok(())
    }

    pub fn context_exists(&self, context_name: &String) -> bool {
        match self.task_file_paths.get(context_name) {
            Some(_) => true,
            None => false,
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
            information: Some(String::new()),
        };
        let task_2 = Task {
            description: String::from("Study for exam"),
            state: TaskState::Todo,
            category: Option::Some(String::from("School")),
            information: Some(String::new()),
        };
        let task_3 = Task {
            description: String::from("Get haircut"),
            state: TaskState::Todo,
            category: Option::Some(String::from("Personal")),
            information: Some(String::new()),
        };
        let task_4 = Task {
            description: String::from("Workout"),
            state: TaskState::Todo,
            category: Option::None,
            information: Some(String::new()),
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
