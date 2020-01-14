pub mod errors;
mod formatting;
mod objects;
pub mod settings;
mod state;

use itertools::{rev, sorted};

type EfficacyResult<T> = Result<T, errors::EfficacyError>;

#[derive(Debug)]
pub struct Efficacy<'a> {
    config: &'a settings::Settings,
    state: state::State<'a>,
}

impl<'a> Efficacy<'a> {
    pub fn init(config: &'a settings::Settings) -> EfficacyResult<Efficacy<'a>> {
        Ok(Efficacy {
            config,
            state: match state::State::new(&config) {
                Ok(s) => s,
                Err(e) => return Err(e),
            },
        })
    }
}

// Task Operations
impl<'a> Efficacy<'a> {
    pub fn add_task(
        &mut self,
        description: String,
        category: Option<String>,
    ) -> EfficacyResult<()> {
        let new_task = objects::Task {
            category: category,
            description,
            state: objects::TaskState::Todo,
        };

        self.state
            .add_to_category_map(&new_task, self.state.task_objects.len());
        self.state.task_objects.push(new_task);

        self.state.save()
    }

    pub fn complete_task(&mut self, id: usize) -> EfficacyResult<()> {
        match self.state.task_objects.get_mut(id) {
            Some(t) => {
                t.state = objects::TaskState::Done;
            }
            None => return Err(errors::EfficacyError::MismatchedIdError),
        }

        self.state.save()
    }

    pub fn edit_task(
        &mut self,
        id: usize,
        new_description: Option<String>,
        new_category: Option<String>,
    ) -> EfficacyResult<()> {
        let original_task = match self.state.task_objects.get_mut(id) {
            Some(t) => t,
            None => return Err(errors::EfficacyError::MismatchedIdError),
        };

        match new_description {
            Some(d) => original_task.description = d,
            None => (),
        }

        match new_category {
            Some(c) => {
                original_task.category = Some(c);
                self.state.rebuild_category_map();
            }
            None => (),
        }

        self.state.save()
    }

    pub fn delete_task(&mut self, id: usize) -> EfficacyResult<objects::Task> {
        if id >= self.state.task_objects.len() {
            println!("ID not valid");
            return Err(errors::EfficacyError::MismatchedIdError);
        }

        let deleted_task = self.state.task_objects.remove(id);

        self.state.rebuild_category_map();
        self.state.save()?;

        Ok(deleted_task)
    }
}

// Category Operations
impl<'a> Efficacy<'a> {
    pub fn edit_category(
        &mut self,
        category: String,
        new_category_title: String,
    ) -> EfficacyResult<()> {
        let ids_to_move = match self.state.category_map.get(&category) {
            Some(ids) => ids.clone(),
            None => return Err(errors::EfficacyError::NonexistentCategoryError),
        };
        for id in ids_to_move {
            let task = match self.state.task_objects.get_mut(id) {
                Some(t) => t,
                None => return Err(errors::EfficacyError::MismatchedIdError),
            };
            task.category = Some(new_category_title.clone());
        }

        self.state.rebuild_category_map();
        self.state.save()
    }

    pub fn delete_category(
        &mut self,
        category: Option<String>,
    ) -> EfficacyResult<()> {
        let category = match category {
            Some(s) => s,
            None => String::from("No category"),
        };

        let tasks_to_delete = match self.state.category_map.get(&category) {
            Some(v) => v,
            None => return Err(errors::EfficacyError::NonexistentCategoryError),
        };

        for id in rev(sorted(tasks_to_delete.iter())) {
            self.state.task_objects.remove(*id);
        }

        self.state.rebuild_category_map();
        self.state.save()
    }
}

// Cleaning Operations
impl<'a> Efficacy<'a> {
    pub fn clean(&mut self) -> EfficacyResult<()> {
        self.state.task_objects = self
            .state
            .task_objects
            .iter()
            .filter(|t| t.state == objects::TaskState::Todo)
            .cloned()
            .collect();

        self.state.rebuild_category_map();
        self.state.save()
    }
}

// Context operations
impl<'a> Efficacy<'a> {
    pub fn new_context(&mut self, context_name: &String) -> EfficacyResult<()> {
        self.state.new_context(context_name)
    }

    pub fn change_context(&mut self, context_name: &String) -> EfficacyResult<()> {
        self.state.change_context(context_name)
    }

    pub fn context_exists(&self, context_name: &String) -> bool {
        self.state.context_exists(context_name)
    }

    pub fn delete_context(&mut self, context_name: &String) -> EfficacyResult<()> {
        self.state.delete_context(context_name)
    }
}

// Listing Operations
impl<'a> Efficacy<'a> {
    pub fn list(&self) -> EfficacyResult<String> {
        let mut result = String::from("\n");

        for (category, ids) in sorted(self.state.category_map.iter()) {
            let category_line = formatting::format_category(category, ids);
            result.push_str(&(category_line + "\n"));

            if ids.is_empty() {
                result.push_str(&formatting::format_nothing());
            }

            let ids_sorted_by_state = sorted(ids.iter().map(|id| {
                let task = match self.state.task_objects.get(*id) {
                    Some(t) => t,
                    None => panic!("State is not good!"),
                };
                (&task.state, id)
            }));

            for (_, task_id) in ids_sorted_by_state {
                let task = match self.state.task_objects.get(*task_id) {
                    Some(t) => t,
                    None => return Err(errors::EfficacyError::MismatchedIdError),
                };

                let task_line =
                    formatting::format_task(&self.config.task_format, &task, *task_id) + "\n";
                result.push_str(&task_line);
            }

            result.push_str("\n");
        }

        if result.eq("\n") {
            result = formatting::format_nothing();
        } else {
            result.pop();
        }

        Ok(result)
    }

    pub fn list_contexts(&self) -> EfficacyResult<String> {
        let mut result = String::from("\n");

        for key in sorted(self.state.task_file_paths.keys()) {
            let context_line = formatting::format_context(key, key.eq(&self.state.current_context.context_name)) + "\n";
            result.push_str(&context_line);
        }

        Ok(result)
    }

    // Debug information
    pub fn debug(&self) -> EfficacyResult<()> {
        println!("Context:");
        println!("{:#?}", self.state.current_context);

        println!("Task objects:");
        println!("{:#?}", self.state.task_objects);

        println!("Category map:");
        println!("{:#?}", self.state.category_map);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn format_objects() {
        use super::formatting;
        use super::objects::{Task, TaskState};

        // Check task formatting
        let task = Task {
            description: String::from("Add classes to calendar"),
            state: TaskState::Done,
            category: Option::Some(String::from("School")),
        };
        let task_fmt_string = String::from("%b %d (#%i)");

        assert_eq!(
            "[X] Add classes to calendar (#1)\n",
            formatting::format_task(&task_fmt_string, &task, 1)
        );
    }
}
