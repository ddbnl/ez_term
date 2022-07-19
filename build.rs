//! This build file generates a .rs module based on the .ez config files. This way the compiled
//! binary does not depend on the .ez config files but is instead baked in.
use std::collections::HashMap;
use std::env;
use std::fs::{File, read_dir};
use std::io::prelude::*;
use std::fs;
use std::path::Path;

fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("ez_file_gen.rs");

    let ez_folder = get_ez_folder();
    println!("cargo:rerun-if-changed={}", ez_folder);

    let files = load_ez_folder(ez_folder.as_str());
    let mut gen = "\
pub fn ez_config() -> HashMap<String, String> {
    let mut files = HashMap::new();\n".to_string();
    for (path, content) in files {
        gen = format!("{}\n\
    files.insert(r\"{}\".to_string(), \"{}\".to_string());", gen, path, content);
    }
    gen = format!("{}\
    files\n\
    }}", gen);
    fs::write(&dest_path, gen).unwrap();

    // Hack to always rebuild, as cargo might nog recognize changes in only .ez files
    env::set_var("REBUILD", format!("{:?}", std::time::Instant::now()));
    println!("cargo:rerun-if-env-changed=REBUILD");
}


fn get_ez_folder() -> String {

    if env::var("EZ_FOLDER").is_ok() {
        env::var("EZ_FOLDER").unwrap()
    } else {
        panic!("Environment variable \'EZ_FOLDER\' is mandatory and must point to your .ez files.\n\
        On linux: \n \
        export EZ_FOLDER=\"/path/to/ez/files\"\n\
        On Windows:\n\
        $env:EZ_FOLDER=\"/path/to/ez/files\"")
    }
}


/// Load all '.ez' files from a folder recursively. There can only be one root widget, so when
/// loading multiple files make sure all definitions are templates, except for the one root Layout.
pub fn load_ez_folder(folder_path: &str) -> HashMap<String, String> {

    let path = Path::new(folder_path);
    let mut ez_files = Vec::new();
    collect_ez_files(path, &mut ez_files);
    if ez_files.is_empty() {
        panic!("Could not find any .ez files in \"{}\". By default .ez files are loaded from the \
        \"ui\" folder in your project root. To use a custom folder, set the \"EZ_FOLDER\" \
        environment variable before compiling. You can use \"./\" to start from your project root.\n\
        On linux: \n \
        export EZ_FOLDER=\"./path/to/ez/files\"\n\
        On Windows:\n\
        $env:EZ_FOLDER=\"./path/to/ez/files\"", folder_path)
    }
    load_ez_files(ez_files.iter().map(|x| x.as_str()).collect())
}


/// Load multiple file paths into a root layout. Return the root widget and a new scheduler.
/// Both will be needed to run an [App].
pub fn load_ez_files(file_paths: Vec<&str>) -> HashMap<String, String> {

    let mut contents = HashMap::new();
    for path in file_paths {
        let mut file_string = String::new();
        let mut file = File::open(path)
            .unwrap_or_else(|_| panic!("Unable to open file {}", path));
        file.read_to_string(&mut file_string)
            .unwrap_or_else(|_| panic!("Unable to read file {}", path));
        file_string = file_string
            .replace('\\', "\\\\")
            .replace('"', "\\\"")
            .replace('{', "{{")
            .replace('}', "}}");
        contents.insert(path.to_string(), file_string);
    }
    contents
}



/// Find all files that end with '.ez' in a folder recursively.
fn collect_ez_files(dir: &Path, ez_files: &mut Vec<String>) {

    if dir.is_dir() {
        for entry in read_dir(dir).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.is_dir() {
                collect_ez_files(&path, ez_files);
            } else if let Some(extension) = path.extension() {
                if extension == "ez" {
                    ez_files.push(path.to_str().unwrap().to_string());
                }
            }
        }
    }
}
