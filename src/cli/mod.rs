use crate::program;
use clap::App;
use std::error::Error;

pub fn parse() -> Result<(), Box<dyn Error>> {
    let settings = program::settings::Settings::new()?;
    let mut eff = program::Efficacy::init(&settings)?;

    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    // LS command
    if let Some(_) = matches.subcommand_matches("list") {
        println!("{}", eff.list()?);

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
        let description = value_t_or_exit!(matches.value_of("DESCRIPTION"), String);
        match eff.add_task(description, category) {
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

            if description.is_some() || category.is_some() {
                eff.edit_task(id, description, category)?;
                println!("{}", eff.list()?);
            } else {
                println!("")
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
        }

    // CLEAN command
    } else if let Some(_) = matches.subcommand_matches("clean") {
        eff.clean()?;
        println!("{}", eff.list()?);

    // DEBUG command
    } else if let Some(_) = matches.subcommand_matches("debug") {
        eff.debug()?;
    }

    Ok(())
}
