use std::{ fs, io, path::PathBuf, ffi::OsStr };
use std::env;
use std::process::Command;
use std::io::stdin;
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return;
    }
    let dir = &args[1];
    println!("Searching for images in {}", dir);
    let v = list_of_png_paths(dir).unwrap();
    encodefiles(&v);
    println!("{:?}", v[1]);
}

fn list_of_png_paths(root: &str) -> io::Result<Vec<PathBuf>> {
    let mut result = vec![];

    for path in fs::read_dir(root)? {
        let path = path?.path();
        if let Some("png") = path.extension().and_then(OsStr::to_str) {
            result.push(path.to_owned());
        }
    }
    Ok(result)
}

fn encodefiles(files: &Vec<PathBuf>) {
    println!("Select CJXL speed (-e), valid range is 1-9 larger number = slower values past 7 generally useless for lossless");
    let mut speed = String::new();
    stdin()
        .read_line(&mut speed)
        .expect("Failed to read line");
    println!("Select butteraugli distance (-d) lower = higher quality. Valid range: 0-25. 0 is mathematically lossless 1 attempts to be visually lossless in most situations. Fractional values eg: 0.8 are valid");
    let mut distance = String::new();
    stdin()
        .read_line(&mut distance)
        .expect("Failed to read line");
      for file in files{
    let mut dest = file.clone();
            dest.set_extension("jxl");
    Command::new("cjxl")
        .arg("-e")
        .arg(speed.trim())
        .arg("-d")
        .arg(distance.trim())
        .arg(file)
        .arg(dest)
        .spawn()
        .expect("command failed to start");
        }
}
