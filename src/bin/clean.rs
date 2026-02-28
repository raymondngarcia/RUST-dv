use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    // Default behavior: clean everything if no args provided
    let clean_all = args.is_empty() || args.contains(&"--all".to_string());

    if clean_all || args.contains(&"--obj".to_string()) {
        clean_obj_dir();
    }

    if clean_all || args.contains(&"--target".to_string()) {
        clean_target();
    }

    if clean_all || args.contains(&"--sim".to_string()) {
        clean_sim_subdirs();
    }

    println!("Clean complete!");
}

fn clean_obj_dir() {
    let path = Path::new("obj_dir");
    if path.exists() {
        println!("Removing obj_dir...");
        if let Err(e) = fs::remove_dir_all(path) {
            eprintln!("Failed to remove obj_dir: {}", e);
        }
    }
}

fn clean_target() {
    let path = Path::new("target");
    if path.exists() {
        println!("Removing target...");
        if let Err(e) = fs::remove_dir_all(path) {
            eprintln!("Failed to remove target: {}", e);
        }
    }
}

fn clean_sim_subdirs() {
    let sim_path = Path::new("sim");
    if sim_path.exists() {
        println!("Removing subdirectories under sim...");
        for entry in fs::read_dir(sim_path).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.is_dir() {
                if let Err(e) = fs::remove_dir_all(&path) {
                    eprintln!("Failed to remove {:?}: {}", path, e);
                }
            }
        }
    }
}