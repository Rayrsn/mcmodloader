extern crate dirs;
use ansi_term;
use colored::*;
use std::fs;
use std::path::Path;
use std::process::Command;
use std::thread;
use std::time::Duration;
use ureq;

fn main() {
    let colorsupport = ansi_term::enable_ansi_support();
    let appdata = dirs::config_dir().unwrap().to_str().unwrap().to_string();
    let launcher_location = appdata.clone() + "\\.minecraft\\TLauncher.exe";
    let mod_location = appdata.clone() + "\\.minecraft\\mods";
    let base_location = appdata.clone() + "\\modpack";
    let modlist_location = base_location.clone() + "\\mods.list";
    let modver_location = base_location.clone() + "\\modpack.ver";
    let modlist_url = "https://raw.githubusercontent.com/Rayrsn/RayrSMP/main/mods.list";
    let modver_url = "https://raw.githubusercontent.com/Rayrsn/RayrSMP/main/modpack.ver";

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
    let remotever = ureq::get(modver_url)
        .call()
        .into_string()
        .unwrap();
    println!(
        "Local version: {}",
        fs::read_to_string(modver_location.clone()).unwrap()
    );
    println!("Remote version: {}", remotever);
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
            if !Path::new(&fileloc).exists() {
                println!("Downloading: {}", filename);
                let mut filecontent = ureq::get(line).call().into_string().unwrap();
                fs::write(fileloc.clone(), filecontent).unwrap();
            }
        }
        fs::write(modver_location.clone(), remotever).unwrap();
        let newmodver = fs::read_to_string(modver_location.clone()).unwrap();
        let newmodver = newmodver.split("\n").collect::<Vec<&str>>();
        println!("Successfully updated to {}!", newmodver[0].green());
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
