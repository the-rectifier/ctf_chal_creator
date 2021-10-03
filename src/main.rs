extern crate clap;
use clap::{ App, Arg };

use std::fs;
use std::env;
use std::fs::File;
use curl::easy::Easy;
use colored::Colorize;
use std::fs::OpenOptions;
use std::io::{ ErrorKind, Write };

struct ChalConfig {
    name: String,
    author: String,
    chal_type: String,
    docker: bool,
    verbose: bool,
}

fn main() {
    let matches = App::new("Chal Creator")
                        .version("1.0")
                        .author("canopus")
                        .about("I antena Espase i Antena")
                        .arg(Arg::with_name("name")
                            .short("n")
                            .long("name")
                            .value_name("NAME")
                            .takes_value(true)
                            .empty_values(false)
                            .required(true))
                        .arg(Arg::with_name("author")
                            .short("a")
                            .long("author")
                            .value_name("AUTHOR")
                            .empty_values(false)
                            .takes_value(true)
                            .required(true))
                        .arg(Arg::with_name("chal_type")
                            .short("t")
                            .long("type")
                            .value_name("TYPE")
                            .required(true)
                            .possible_values(&["Crypto", "Pwn", "Reversing", "Web", "Misc", "Forensics", "Stego"])
                            .takes_value(true)
                            .empty_values(false))
                        .arg(Arg::with_name("docker")
                            .short("d")
                            .long("docker")
                            .required(false)
                            .takes_value(false))
                        .arg(Arg::with_name("verbose")
                            .short("v")
                            .long("verbose")
                            .required(false)
                            .takes_value(false))
                        .get_matches();

    let chal = ChalConfig {
        name: String::from(matches.value_of("name").unwrap()),
        author: String::from(matches.value_of("author").unwrap()),
        chal_type: String::from(matches.value_of("chal_type").unwrap()),
        docker: matches.is_present("docker"),
        verbose: matches.is_present("verbose"),
    };
    

    create_outer(&chal);
    create_inner(&chal);
    

}

fn create_inner(chal: &ChalConfig) {
    // println!("{}", env::current_dir().unwrap().display());
    env::set_current_dir("Setup").unwrap();
    let inner_files: [&str; 2] = ["Dockerfile", "flag"];

    for (i, file) in inner_files.iter().enumerate() {
        if i < 1 && !chal.docker {
            continue
        }
        match File::create(file) {
            Ok(_) => {
                if chal.verbose {
                    print_info(format!("Created file: {} in Setup/ ", file));
                }
            }
            Err(e) => match e.kind() {
                ErrorKind::NotFound => perror_exit("Path not found!"),
                ErrorKind::PermissionDenied => perror_exit("Permission Denied!"),
                ErrorKind::AlreadyExists => perror_exit("File already Exists!"),
                _ => panic!("Error creating file! {}", e),
            }
        }
    }   

    if chal.chal_type == "Pwn" {
        env::set_current_dir("../Solution").unwrap();
        let mut f = OpenOptions::new()
                    .write(true)
                    .create(true)
                    .open("solution.py")
                    .unwrap();
        
        let mut sol = Easy::new();
        sol.url("https://gist.githubusercontent.com/the-rectifier/9af60d9d85e2600708e582505060258b/raw/8faedc9c67728afa853386aa41a7df873863e0b7/solution.py").unwrap();
        sol.write_function(move |data| {
            f.write_all(data).unwrap();
            Ok(data.len())
        }).unwrap();
        sol.perform().unwrap();

        if chal.verbose {
            print_info("Downloaded solve template!".to_string());
        }
    }
}

fn create_outer(chal: &ChalConfig) {
    let dirs: [&str; 3] = ["Setup", "Public", "Solution"];
    let outside_files: [&str; 3] = ["docker_build.sh", "docker_run.sh", "Readme"];

    match fs::create_dir(&chal.name) { 
        Ok(_) => { 
            if chal.verbose {
                print_info(format!("Challenge Directory {} created successfully!", chal.name));
            }
        }
        Err(e) => match e.kind() { 
            ErrorKind::PermissionDenied => perror_exit("Permission Denied!"),
            ErrorKind::AlreadyExists => perror_exit("Directory already exists!"),
            _ => panic!("Error Creating Directory! {}", e),
        }
    }    

    env::set_current_dir(&chal.name).unwrap();

    for dir in dirs.iter() { 
        match fs::create_dir(dir) { 
            Ok(_) => {
                if chal.verbose {
                    print_info(format!("Subdirectory {} created successfully!", dir));
                }
            }
            Err(e) => match e.kind() { 
                ErrorKind::PermissionDenied => perror_exit("Permission Denied!"),
                ErrorKind::AlreadyExists => perror_exit("Directory already exists!"),
                _ => panic!("Error Creating Directory! {}", e),
            }
        }
    }

    for (i, file) in outside_files.iter().enumerate() { 
        if i < 2 && !chal.docker {
            continue
        }
        match File::create(file) {
            Ok(_) => {
                if chal.verbose {
                    print_info(format!("Created {}!", file));
                }
            }
            Err(e) => match e.kind() {
                ErrorKind::NotFound => perror_exit("Path not found!"),
                ErrorKind::PermissionDenied => perror_exit("Permission Denied!"),
                ErrorKind::AlreadyExists => perror_exit("File already Exists!"),
                _ => panic!("Error creating file! {}", e),
            }
        }
    }

    if chal.docker {
        let mut f = OpenOptions::new()
            .write(true)
            .open(outside_files[0])
            .unwrap();
        
        write!(f, "#!/bin/bash\ndocker build -t {}/{} setup/", chal.author, chal.name).unwrap();

        if chal.verbose { 
            print_info("Created docker_build.sh !".to_string());
        }

        let mut f = OpenOptions::new()
            .write(true)
            .open(outside_files[1])
            .unwrap();
        
        write!(f, "#!/bin/bash\ndocker run -p 69666:69666 -d {}/{}", chal.author, chal.name).unwrap();

        if chal.verbose { 
            print_info("Created docker_run.sh !".to_string());
        }
    }
}



fn perror_exit(msg: &str) {
    println!("{}", msg.red().bold());
    std::process::exit(-1);
}

fn print_info(msg: String) {
    println!("{}", msg.green().bold());
}
