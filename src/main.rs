use std::process::{Command};
use std::fs::{self, File};
use std::io::prelude::*;
use std::io::ErrorKind;

fn start_compiling(flags: String){
    match fs::create_dir("temp") {
        Ok(_o) => {
            println!("Directory created succesfully");
        },
        Err(e) => {
            if e.kind() == ErrorKind::AlreadyExists{
                println!("Directory exists. Skipping...");
            }
            else {
                panic!("There was an error while creating the directory : {}", e);
            }
        }
    }
    
    let file = File::create("temp/compile.bat");
    let command = flags;
    match file {
        Ok(mut o) => {
            println!("File created succesfully");
            o.write_all(command.as_bytes());
        },
        Err(e) => {
            if e.kind() == ErrorKind::AlreadyExists{
                println!("File exists. Skipping...");
            }
            else {
                panic!("There was an error while creating the file : {}", e);
            }
        }
    }    
}

fn settings(){
    println!("Welcome to the BetterF9 for CS2");
    println!("--------------------------------");
    println!("1-) Entities only");
    println!("2-) Pre-Settle physics objects");
    println!("3-) Generate Lightmaps");
    println!("4-) Lightmap noise removal");
    println!("5-) Disable lighting calculations");
    println!("6-) Build physics");
    println!("7-) Build vis");
    println!("8-) Build nav");
    println!("9-) Bake Steam Audio Reverb");
    println!("10-) Bake Steam Audio Paths");
    println!("11-) Change lightmap resolution");
    println!("12-) Change the amount of threads used by Steam Audio and Lightmaps");
    println!("13-) Enable CPU based Lightmap baking");
    println!("14-) Choose your CS2 install path");
    println!("15-) Start compiling");
    println!("16-) Write your maps name");
}

fn main() {
    let mut running = true;
    let mut optionalflags: [bool; 15] = [false; 15];
    let mut lightmapres: String = "512".to_string();
    let mut threads: String = "3".to_string();
    let mut cs2_install_dir = String::new();
    let mut map_name = String::new();
    while running {
        let mut line = String::new();
        settings();
        std::io::stdin().read_line(&mut line).unwrap();
        let line = line.trim();
        match line.parse::<i32>() {
            Ok(o) => {
                match o {
                    1 => {
                        optionalflags[0] = true; //Entities
                    },
                    2 => {
                        optionalflags[1] = true; //Pre-Settle physics objects
                    },
                    3 => {
                        optionalflags[2] = true; //Generate lightmaps
                    },
                    4 => {
                        optionalflags[3] = true; //Lightmap noise removal
                    },
                    5 => {
                        optionalflags[4] = true; //Disable lighting calculations
                    },
                    6 => {
                        optionalflags[5] = true; //Build physics
                    },
                    7 => {
                        optionalflags[6] = true; //Build vis
                    },
                    8 => {
                        optionalflags[7] = true; //Build nav
                    },
                    9 => {
                        optionalflags[8] = true; //Steam Audio reverb
                    },
                    10 => {
                        optionalflags[9] = true; //Steam Audio paths
                    },
                    11 => { // Lightmaps
                        lightmapres = String::new();
                        println!("Please input your desired lightmap resolution");
                        println!("Options : 512, 1024, 2048");
                        std::io::stdin().read_line(&mut lightmapres).unwrap();
                        lightmapres = lightmapres.trim().to_string();
                    },
                    12 => { // Threads
                        threads = String::new();
                        println!("Please input the amount of threads to use for lightmaps and Steam Audio");
                        println!("It is recommended that you put the full amount of threads you have on your processor");
                        println!("This wont make a huge difference if you bake Lightmaps with your GPU");
                        std::io::stdin().read_line(&mut threads).unwrap();
                        threads = threads.trim().to_string();
                    },
                    13 => { // Bake lightmaps with CPU
                        optionalflags[10] = true; //Lightmapcpu
                    },
                    14 => { // Installation directory
                        cs2_install_dir = String::new();
                        println!("Please input your path to your Counter-Strike Global Offensive folder");
                        std::io::stdin().read_line(&mut cs2_install_dir).unwrap();
                        optionalflags[11] = true; //Check for install_dir
                    },
                    15 => { // Running
                        if optionalflags[11] == true && optionalflags[12] == true {
                            running = false;
                        }
                        else {
                            println!("Please set your install directory or map name");
                        }
                    },
                    16 => { // Running
                        println!("Please write your maps name");
                        std::io::stdin().read_line(&mut map_name).unwrap();
                        optionalflags[12] = true; // Check if map name is entered or not
                    },
                    i32::MIN..=0_i32 | 17_i32..=i32::MAX => todo!()
                }
            },
            Err(e) => {
                panic!("Error occured : {}", e);
            }
        }
    }
    let mut flags = String::new();
    let final_dir = r#"""#.to_owned() + &cs2_install_dir.clone().trim() + r#"\game\bin\win64\resourcecompiler.exe"# + r#"" "#;
    flags += &final_dir;
    flags += "-unbufferedio -fshallow -retail -nop4 ";
    let mut usable_dir = String::new();
    if optionalflags[11] == true {
        usable_dir = cs2_install_dir.to_lowercase().replace(r#"\"#, "/");
        let map_dir = format!("/content/csgo_addons/{}/maps/{}.vmap", map_name.trim(), map_name.trim());
        let ihatethis = r#"""#.to_owned() + &usable_dir.clone().trim() + map_dir.trim() + r#"""#;
        flags += &format!("-i {} ", ihatethis.trim());
        let out_root = r#"""#.to_owned() + &usable_dir.clone().trim()+ "/game" + r#"""#;
        flags += &format!("-outroot {} ", out_root.trim());
        print!("{}", usable_dir);
    }
    if optionalflags[0] == true {
        flags += "-entities ";
    }
    else {
        flags += "-world "
    }
    if optionalflags[1] == false{
        flags += "-nosettle "
    }
    if optionalflags[2] == true {
        let lightmap = &format!(
            "-bakelighting -lightmapMaxResolution {} -lightmapDoWeld -lightmapVRadQuality 1 -lightmapLocalCompile ",
            lightmapres);
        flags += lightmap.trim();
    }
    if optionalflags[3] == false {
        flags += "-lightmapDisableFiltering "
    }
    if optionalflags[4] == true {
        flags += "-disableLightingCalculations "
    }
    if optionalflags[5] == true {
        flags += "-phys "
    }
    if optionalflags[6] == true {
        flags += "-vis "
    }
    if optionalflags[7] == true {
        flags += "-nav "
    }
    if optionalflags[8] == true {
        let sa = &format!(
            "-sareverb -sareverb_threads {} " , threads
        );
        flags += sa.trim();
    }
    if optionalflags[9] == true {
        let sa2 = &format!(
            "-sapaths -sareverb_threads {} " , threads
        );
        flags += sa2.trim();
    }
    if optionalflags[10] == true {
        flags += "-lightmapcpu ";
    }
    start_compiling(flags);
}

