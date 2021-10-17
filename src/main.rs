use anyhow::{ Context, Result };
use log::{ error, info };
use std::fs::{ self, File, OpenOptions };
use simplelog::{ ColorChoice, TermLogger, TerminalMode };
use std::io::{ self, Write };
use std::path::{ PathBuf };
use structopt::StructOpt;
use clap::arg_enum;


arg_enum! {
    #[derive(Debug, PartialEq, Eq)]
    enum ChalType { 
        Pwn,
        Reverse,
        Crypto,
        Web,
        Misc,
        Forensics,
        Stego,
    }
}

#[derive(Debug)]
struct ChalConfig {
    name: String,
    author: String,
    directory: String,
    chal_type: ChalType,
    docker: bool,
    verbose: bool,
}

#[derive(Debug, StructOpt)]
#[structopt(
    name = "Ctf Chal Creator",
    author = "canopus",
    about = "I antena espase i antena"
)]
struct Opts {
    #[structopt(long, short)]
    name: String,

    #[structopt(long, short)]
    author: String,
    
    #[structopt(long, short = "t", possible_values = &ChalType::variants(), case_insensitive = true)]
    chal_type: ChalType,

    #[structopt(long, short)]
    docker: bool,

    #[structopt(long, short)]
    verbose: bool,

    #[structopt(long, short = "p")]
    dir: String,

}


fn main() -> Result<()> {
    let opts = Opts::from_args();

    let chal = ChalConfig {
        name: opts.name,
        author: opts.author,
        chal_type: opts.chal_type,
        docker: opts.docker,
        verbose: opts.verbose,
        directory: opts.dir,
    };

    let log_level = match chal.verbose { 
        true => log::LevelFilter::Info,
        false => log::LevelFilter::Warn,
    };

    TermLogger::init(
        log_level,
        simplelog::Config::default(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    ).expect("Failed to init logger");

    if let Err(e) = run(&chal) {
        error!("{:?}", e);
        std::process::exit(-1);
    }
    

    Ok(())
}


fn run(chal: &ChalConfig) -> Result<()> {
    create_outer(chal)?;
    create_inner(chal)?;

    Ok(())
}

fn create_inner(chal: &ChalConfig) -> Result<()> {
    let inner_files: [&str; 2] = ["Dockerfile", "flag"];

    for (i, file) in inner_files.iter().enumerate() {
        if i < 1 && !chal.docker {
            continue;
        }

        let path = PathBuf::from(format!("{}/{}/Setup", chal.directory, chal.name)).join(file);
        File::create(&path)
            .with_context(|| format!("Failed to Create '{}'", path.display()))?;

        info!("Created '{}'", path.display());
    }   

    if chal.chal_type == ChalType::Pwn {
        let http_req = ureq::get("https://gist.githubusercontent.com/the-rectifier/9af60d9d85e2600708e582505060258b/raw/8faedc9c67728afa853386aa41a7df873863e0b7/solution.py")
                            .call()
                            .context("Failed to fetch solution template")?;

        let path = PathBuf::from(format!("{}/{}/Solution", chal.directory, chal.name)).join("solution.py");

        let mut f = File::create(path)
                        .with_context(|| format!("Failed to create 'Solution/solution.py'"))?;

        io::copy(&mut http_req.into_reader(), &mut f)
            .with_context(|| format!("Failed to write Solution/solution.py"))?;
        
        info!("Downloaded Solve template!");
    }

    Ok(())
}

fn create_outer(chal: &ChalConfig) -> Result<()> {
    let dirs: [&str; 3] = ["Setup", "Public", "Solution"];
    let readme = "Readme";
    let docker_files: [&str; 2] = ["docker_build.sh", "docker_run.sh"];
    let chal_dir = PathBuf::from(format!("{}/{}", chal.directory, chal.name));

    fs::create_dir(&chal_dir)
        .with_context(|| format!("Failed to create '{}/' ", chal_dir.display()))?;

    info!("Directory '{}/' created successfully!", chal_dir.display());

    
    for dir in dirs.iter() {
        let path = chal_dir.join(dir);

        fs::create_dir(&path)
            .with_context(|| format!("Failed to create '{}' subdirectory", path.display()))?;
        
        info!("Subdirectory '{}' created!", path.display());
    }

    let path = chal_dir.join(readme);
    File::create(&path)
        .with_context(|| format!("Failed to create '{}'", path.display()))?;
    info!("Created '{}'", path.display());

    if chal.docker { 
       let path = chal_dir.join(docker_files[0]);
       let mut f = OpenOptions::new()
                    .create(true)
                    .write(true)
                    .mode(0o755)
                    .open(&path)
                    .with_context(|| format!("Failed to create {}", path.display()))?;
    
        write!(
            f,
            "#!/usr/bin/bash\ndocker build -t {}/{} Setup/",
            chal.author, chal.name
        )
        .with_context(|| format!("Couldn't write to '{}'", path.display()))?;

        info!("Created {}", path.display());

        let path = chal_dir.join(docker_files[1]);
        let mut f = OpenOptions::new()
                    .create(true)
                    .mode(0o755)
                    .write(true)
                    .open(&path)
                    .with_context(|| format!("Failed to create {}", path.display()))?;
    
        write!(
            f,
            "#!/usr/bin/bash\ndocker run -p 69666:69666 -d {}/{}",
            chal.author, chal.name
        )
        .with_context(|| format!("Couldn't write to '{}'", path.display()))?;

        info!("Created {}", path.display());
    }

    info!("Outer files created successfully!");
    Ok(())
}
