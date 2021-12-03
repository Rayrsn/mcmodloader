extern crate dirs;
use ansi_term;
use colored::*;
use std::fs;
use std::path::Path;
use std::process::Command;
use std::thread;
use std::time::Duration;
use ureq;
use std::io::Cursor;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;
 
async fn fetch_url(url: String, file_name: String) -> Result<()> {
    let response = reqwest::get(url).await?;
    let mut file = std::fs::File::create(file_name)?;
    let mut content =  Cursor::new(response.bytes().await?);
    std::io::copy(&mut content, &mut file)?;
    Ok(())
}
#[tokio::main]
async fn main() {
    let colorsupport = ansi_term::enable_ansi_support();
    let appdata = dirs::config_dir().unwrap().to_str().unwrap().to_string();
    let launcher_location = appdata.clone() + "\\.minecraft\\TLauncher.exe";
    let mod_location = appdata.clone() + "\\.minecraft\\mods";
    let base_location = appdata.clone() + "\\modpack";
    let modlist_location = base_location.clone() + "\\mods.list";
    let modver_location = base_location.clone() + "\\modpack.ver";
    let modlist_url = "https://raw.githubusercontent.com/Rayrsn/Rayr-Origins-SMP/main/mod.list";
    let modver_url = "https://raw.githubusercontent.com/Rayrsn/Rayr-Origins-SMP/main/mod.ver";
    let mut count = 0;

    // check wether base_location exists if not create it
    if !Path::new(&base_location).exists() {
        std::fs::create_dir_all(&base_location).unwrap();
    }
    // check wether modlist_location exists if not create it
    if !Path::new(&modlist_location).exists() {
        std::fs::File::create(&modlist_location).unwrap();
    }
    // check wether modver_location exists if not create it
    if !Path::new(&modver_location).exists() {
        std::fs::File::create(&modver_location).unwrap();
    }
    println!("{}", "Fetching versions...".cyan());
    println!(
        "Local version: {}",
        fs::read_to_string(modver_location.clone()).unwrap()
    );
    
    print!("Remote version: ");
    let remotever = ureq::get(modver_url)
        .call()
        .into_string()
        .unwrap();
    println!("{}", remotever);
    // if remotever is equal to the current modpack.ver then do nothing
    if remotever != fs::read_to_string(modver_location.clone()).unwrap() {
        println!("{}", "Updating...".yellow());
        // get modlist and write it to modlist_location
        let modlist = ureq::get(modlist_url)
            .call()
            .into_string()
            .unwrap();
        fs::write(modlist_location.clone(), modlist).unwrap();
        for line in fs::read_to_string(modlist_location.clone())
            .unwrap()
            .lines()
        {
            let filename = line.split("/").last().unwrap();
            let mut fileloc = (mod_location.clone() + "\\" + filename).to_string().clone();
            count += 1;
            if !Path::new(&fileloc).exists() {
                println!("{}.Downloading: {}",count ,filename);
                fetch_url(line.to_string(), fileloc.clone().to_string()).await.unwrap();
            }
        }
        fs::write(modver_location.clone(), remotever).unwrap();
        let newmodver = fs::read_to_string(modver_location.clone()).unwrap();
        let newmodver = newmodver.split("\n").collect::<Vec<&str>>();
        println!("\nSuccessfully updated to {}!", newmodver[0].green());
    } else {
        println!("{}", "No update needed!".green());
    }
    println!("\n{}", "Launching...".blue());
    let launchthread = thread::spawn(|| {
        thread::sleep(Duration::from_secs(5));
        std::process::exit(1);
    });
    Command::new("cmd.exe")
        .arg("/c")
        .arg(launcher_location)
        .spawn()
        .unwrap();
    launchthread.join().unwrap();
}
