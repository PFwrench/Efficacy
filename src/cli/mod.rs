use crate::program;
use clap::App;
use std::error::Error;

pub fn parse() -> Result<(), Box<dyn Error>> {
    let settings = program::settings::Settings::new()?;
    let mut eff = program::Efficacy::init(&settings)?;

    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    // LS command
    if let Some(matches) = matches.subcommand_matches("list") {
        if matches.is_present("context") {
            println!("{}", eff.list_contexts()?);
        } else {
            match matches.value_of("ID") {
                Some(id) => {
                    let id: usize = id.parse()?;
                    let task_string = eff
                        .list_task(id)
                        .unwrap_or(String::from("Invalid ID provided"));
                    println!("{}", task_string);
                }
                None => println!("{}", eff.list()?),
            }
        }

    // DONE command
    } else if let Some(matches) = matches.subcommand_matches("done") {
        let id = value_t_or_exit!(matches.value_of("ID"), usize);
        eff.complete_task(id)?;
        println!("{}", eff.list()?);

    // ADD command
    } else if let Some(matches) = matches.subcommand_matches("add") {
        let category = match matches.value_of("category") {
            Some(s) => Option::Some(String::from(s)),
            None => Option::None,
        };
        let information = match matches.value_of("information") {
            Some(s) => Some(String::from(s)),
            None => None,
        };
        let description = value_t_or_exit!(matches.value_of("DESCRIPTION"), String);
        match eff.add_task(description, category, information) {
            Ok(_) => println!("{}", eff.list()?),
            Err(_) => println!("There was an error in creating the new task."),
        }

    // EDIT command
    } else if let Some(matches) = matches.subcommand_matches("edit") {
        if let Some(matches) = matches.subcommand_matches("task") {
            let id = value_t_or_exit!(matches.value_of("ID"), usize);
            let description = match matches.value_of("description") {
                Some(s) => Some(String::from(s)),
                None => None,
            };
            let category = match matches.value_of("category") {
                Some(s) => Some(String::from(s)),
                None => None,
            };
            let information = match matches.value_of("information") {
                Some(s) => Some(String::from(s)),
                None => None,
            };

            if description.is_some() || category.is_some() || information.is_some() {
                eff.edit_task(id, description, category, information)?;
                println!("{}", eff.list()?);
            } else {
                println!("No new information provided.");
            }
        } else if let Some(matches) = matches.subcommand_matches("category") {
            let old_title = value_t_or_exit!(matches.value_of("OLD_TITLE"), String);
            let new_title = value_t_or_exit!(matches.value_of("NEW_TITLE"), String);

            eff.edit_category(old_title, new_title)?;
            println!("{}", eff.list()?);
        }

    // DELETE commands
    } else if let Some(matches) = matches.subcommand_matches("delete") {
        if let Some(matches) = matches.subcommand_matches("task") {
            let id = value_t_or_exit!(matches.value_of("ID"), usize);
            match eff.delete_task(id) {
                Ok(_) => println!("{}", eff.list()?),
                Err(_) => println!("There was an error in deleting the task"),
            }
        } else if let Some(matches) = matches.subcommand_matches("category") {
            let title = match matches.value_of("TITLE") {
                Some(s) => Option::Some(String::from(s)),
                None => Option::None,
            };

            match eff.delete_category(title) {
                Ok(_) => println!("{}", eff.list()?),
                Err(e) => println!("There was an error in deleting the category: {}", e),
            }
        } else if let Some(matches) = matches.subcommand_matches("context") {
            let context = value_t_or_exit!(matches.value_of("CONTEXT"), String);
            match eff.delete_context(&context) {
                Ok(_) => println!("Context '{}' deleted successfully.", context),
                Err(e) => println!("Error when deleting context: {:?}", e),
            }
        }

    // CLEAN command
    } else if let Some(_) = matches.subcommand_matches("clean") {
        eff.clean()?;
        println!("{}", eff.list()?);

    // CONTEXT command
    } else if let Some(matches) = matches.subcommand_matches("context") {
        let context = match matches.value_of("CONTEXT") {
            Some(s) => String::from(s),
            None => String::from("default"),
        };
        if matches.is_present("new") {
            match eff.new_context(&context) {
                Ok(_) => {
                    println!(
                        "Context '{}' created successfully! Switched to '{0}'",
                        context
                    );
                    return Ok(());
                }
                Err(_) => return Ok(()),
            }
        } else {
            match eff.change_context(&context) {
                Ok(_) => (),
                Err(_) => return Ok(()),
            }
        }

        println!("{}", eff.list()?);

    // DEBUG command
    } else if let Some(_) = matches.subcommand_matches("debug") {
        eff.debug()?;
    }

    Ok(())
}
