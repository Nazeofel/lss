use std::{fs, io, env, path::{Path, PathBuf}, ffi::OsStr};
use colored::Colorize;

fn main(){
    let current_dir = env::current_dir().expect("Couldn't find the path");
    let entries = fs::read_dir(current_dir).expect("Couldn't read current dir")
        .map(|res: Result<fs::DirEntry, io::Error>| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>().expect("Collection has faield");

    if entries.len() <= 0{
        println!("{}", "There aren't any files or folder in this directory".red());
        return;
    }

    let file_size_string_length = file_size_string_length(&entries);
    
    match pretty_print(file_size_string_length, &entries)  {
        Ok(_) => {},
        Err(e) => {
            println!("{} {}", "Error:".red(), e.to_string().red());
        }
    }
}


fn pretty_print(longest_size: usize, files: &Vec<PathBuf>) -> io::Result<()>{

    println!("");
    println!("{}   {}{}{}", " ".repeat(4), "Size".yellow(), " ".repeat(longest_size - 1), "Name".yellow());

    let mut number_of_dir: i32 = 0;
    let mut number_of_files: i32 = 0;

    for file in files {

        let file_size: u64 = fs::metadata(file)?.len();
        let file_parsed_path = Path::new(&file).file_name().and_then(OsStr::to_str).expect("wrong path").to_string();

        let spacing: usize = {
            if longest_size - file_size.to_string().len() <= 0 {
                0
            } else {
                longest_size - file_size.to_string().len() + 3
            }
        };

        if file.is_dir(){
            if file_size.to_string().len() < longest_size {
                println!("{}   {}{}{}", "|  " .cyan(), file_size, " ".repeat(spacing), file_parsed_path.cyan());
            } else {
                println!("{}   {}   {}", "|  " .cyan(), file_size, file_parsed_path.cyan());
            }
            number_of_dir = number_of_dir + 1;
        } else {
            if file_size.to_string().len() < longest_size {
                println!("{}   {}{}{}", "|  ".green(), file_size, " ".repeat(spacing), file_parsed_path.green());   
            } else {
                println!("{}   {}   {}", "|  ".green(), file_size, file_parsed_path.green());
            }
            number_of_files = number_of_files + 1;
        }
    }

    println!(" ");
    println!(" Total : {} - {} | {} - {} \n", "  ".cyan(), number_of_dir, "  ".green(), number_of_files);

    
    Ok(())
}
fn file_size_string_length(files: &Vec<PathBuf>) -> usize {
    let mut length: usize = 0;
    for file in files {
        let file_metadata = fs::metadata(file);
        match file_metadata {
            Ok(res) => {
                let file_size = res.len();
                if file_size.to_string().len() > length {
                    length = file_size.to_string().len() as usize;
                }
            },
            Err(e) => {
                println!("{} {}", "Error:".red(), e.to_string().red());
            }
        }
    }
    length
}
