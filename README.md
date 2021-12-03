# Minecraft Auto Mod Downloader (Rust rewrite)
### A Rust program which automatically downloads mods from a given list.
* This program is a rework of my original [project](https://github.com/Rayrsn/Minecraft-Auto-Mod-Downloader) written entirely in rust
# Requirements
1. Upload your version of `mod.list` and `mod.ver`. (I suggest uploading them to a github repo because the script requires direct links to the raw file.)
2. The `mods.list` file has to only include the ***Direct*** links to the mods (basically it has to look like this):

![mods.list](https://github.com/Rayrsn/Minecraft-Auto-Mod-Downloader/raw/main/images/mods.list.png?raw=true)

3. The `modpack.ver` file also has to only include the version and it has to look like this:

![modpack.ver](https://github.com/Rayrsn/Minecraft-Auto-Mod-Downloader/raw/main/images/modpack.ver.png?raw=true)

## Setting up and Running the program from source
1. Open `main.rs` with a text editor.
2. Edit lines <ins> 30 </ins> and <ins> 31 </ins> (`modlist_url`, `modver_url`) to the correct values. (Read [Requirements](https://github.com/Rayrsn/mcmodloader/blob/main/README.md#requirements).)
3. Change line <ins> 25 </ins> to your preferred launcher. (You might have to change the `AppData` values if the launcher is ***not*** located in the `AppData/Roaming/.minecraft` directory.)
4. Run the program without building by running in the main directory:
```bash
cargo run
```
## Making an executable (.exe file)
* For building a better executable (in terms of performance and size) follow step 1:
1.
```bash
cargo build --release
```
* If you want the program to have a minecraft icon follow step 2:
2.
```bash
cargo rustc --release -- -C link-args="resources.res"
```

### Your file will be located in target/release directory
