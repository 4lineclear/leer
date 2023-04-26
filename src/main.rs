use std::{
    error::Error,
};

// TODO:https://stackoverflow.com/questions/59890270/how-do-i-overwrite-console-output
// TODO:https://www.scrabblepages.com/scrabble/rules/
//TODO:https://www.wordgamedictionary.com/sowpods/download/sowpods.txt
fn main()  -> Result<(), Box<dyn Error>>  {
    // test()?;
    scrabble_model::config::load()?;


    Ok(())

    // dbg!(current_exe().unwrap());
    // load().expect("Missing config folder");
}

