use std::{path::PathBuf, env, fs};

use crate::{commands::repository::{RepositoryArgs,RepositoryCommands, ExcludeArgs, CheckoutArgs, SaveArgs}, DATABASE};

pub fn handle(args: RepositoryArgs){
    match args.commands{
        RepositoryCommands::List => list(),
        RepositoryCommands::Exclude(args) => exclude(args),
        RepositoryCommands::Add => add(),
        RepositoryCommands::Checkout(args) => checkout(args),
        RepositoryCommands::Save(args) => save(args),
    }
}

fn list(){
    let db = DATABASE.lock().unwrap();
    db.iter().for_each(|r| println!("Identifier: {}, Volume: {}", r.history_id, r.entries.len()));
}

fn exclude(args: ExcludeArgs){
    let mut db = DATABASE.lock().unwrap();

    let excluded = match args.index {
        Some(id) => db.remove_history_by_id(id),
        None => db.remove_history(),
    };

    match excluded{
        Some(excluded) => println!("History {} removed!", excluded.history_id),
        None => println!("There is no history with id {}!", args.index.unwrap_or(db.selected_history_id))
    };
}

fn add(){
    let mut db = DATABASE.lock().unwrap();
    //let id = db.new_entry();

    //println!("Repository {} added!", id);
}

fn checkout(args: CheckoutArgs){
    let mut db = DATABASE.lock().unwrap();
    match db.change_selected(args.index){
        Some(history) =>println!("Current working image history repository changed to {}", history.history_id),
        None => println!("There is no history with id {}!", args.index)
    };
}

fn save(args: SaveArgs){
    let mut db = DATABASE.lock().unwrap();

    let mut root_path = match args.index{
        Some(path) => PathBuf::from(path),
        None => env::current_dir().unwrap()
    };

    if root_path.is_dir() == false {
        println!("There is no path at {}", root_path.to_string_lossy());
        return;
    }

    root_path.push("pid_data");

    if(root_path.is_dir()){
        println!("There is already data at {}!", root_path.into_os_string().into_string().unwrap());
        return;
    }

    for history in db.repository.iter_mut(){
        let mut history_dir = root_path.clone();

        history_dir.push(history.history_id.to_string());

        for (index, entry) in history.entries.iter_mut().enumerate(){
            fs::create_dir_all(history_dir.clone());
            let mut image_path = history_dir.clone();
            image_path.push(format!("{}_{}",index, entry.uuid.to_string()));
            image_path.set_extension("png");

            let status = entry.image.save(image_path.clone());
            match status{
                Ok(_) => println!("Image {} from history {} saved at {}!", entry.uuid, history.history_id, image_path.into_os_string().into_string().unwrap()),
                Err(err) => println!("Cannot save image {} from history {}! Error: {}", entry.uuid, history.history_id, err.to_string()),
            }
        }
    }

}