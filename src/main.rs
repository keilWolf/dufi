use std::{fs::{self, ReadDir}, collections::HashMap, io::{self, Write}, path::PathBuf };
use clap::Parser;

/// Search duplicates in a src directory and trash
/// duplicates to thrash directory.
#[derive(Parser)]
struct Cli {
    /// The source where to look for duplicates
    src: std::path::PathBuf,
    /// The trash directory where to store duplicate items
    trash_dir: std::path::PathBuf,
}

/// Create hash for each file in directory and stores it in a hashmap.
/// The hashmap uses the hashstring as key and a vector as value
/// containing all files.
fn create_hashes(f: ReadDir) -> HashMap<String, Vec<String>> {
    let mut hashs: HashMap<String, Vec<String>> = HashMap::new();
    let mut count: usize = 0;
    for ele in f {
        count += 1;
        print!("\r[{}]", count);
        io::stdout().flush().ok().expect("Could not flush");


        let v = ele.unwrap();
        let content = fs::read(v.path()).expect("should be a readable file");
        let hash_string = blake3::hash(&content).to_string();
        let file_name = v.file_name().to_str().unwrap().to_string();

        if !hashs.contains_key(hash_string.as_str()){
            hashs.insert(hash_string.clone(), Vec::new());
        }

        let a = hashs.get(&hash_string).unwrap();
        let b = vec![file_name];
        let mut c: Vec<String> = Vec::new();
        for ele in a {
            c.push(ele.to_string());
        }
        for ele in b {
            c.push(ele.to_string());
        }

        hashs.insert(hash_string.clone(), c);
    }
    return hashs
}

// Iter through hashs and count hashs with more than one file.
fn get_number_of_duplicates(hashs: &HashMap<String, Vec<String>>) -> usize {
    return hashs.iter().fold(0, |acc, elem| {
        let (_, value) = elem;
        if value.len() > 1 {
            acc + 1
        }else{
            acc
        }
    });
}

fn ask_user_which_to_keep(files: &Vec<String>, src: &PathBuf, trash_dir: &PathBuf) {
    let num_files = files.len();
    if num_files > 1 {
        println!("Which file do you want to keep?");
        println!("==============================?");
        println!("");
        for i in 0..num_files {
            let file = &files[i];
            println!("({i}) ... {file}");
        }

        let mut user_input = String::new();
        println!("Enter number: ");
        std::io::stdin().read_line(&mut user_input).unwrap();

        match user_input.trim().parse::<usize>() {
            Err(_) => println!("This was not an integer ... skipped"),
            Ok(v) => {
                if v < num_files {
                    print!("Keep ... {} ", files[v]);
                    let mut problem_occured = false;
                    for i in 0..num_files{
                        if i != v {
                            let name = files[i].as_str();
                            let src = src.join(name);
                            let dst = trash_dir.join(name);
                            match std::fs::rename(src, dst){
                                Ok(_) => {},
                                Err(e) => {
                                    print!("\n");
                                    println!("❌ failed to move to {} ... {e}.", trash_dir.to_str().unwrap());
                                    problem_occured = true;
                                },
                            };
                        };
                    }
                    if !problem_occured {
                        print!("✅\n");
                    }
                }else{
                    println!("Number should be between [0-{}] ... skipped", num_files-1);
                }
            },
        };
    }
}



fn main(){
    println!("DuFi --- Duplication Finder");

    let args = Cli::parse();
    let f = fs::read_dir(&args.src).unwrap();
    
    let hashs = create_hashes(f);
    let num_duplicates = get_number_of_duplicates(&hashs);

    let mut count = 0;
    for (_, files) in &hashs {
        if files.len() > 1 {
            count += 1;
            println!("");
            println!("           [{}/{}]", count, num_duplicates);
            ask_user_which_to_keep(files, &args.src, &args.trash_dir);
        }
    }
}
