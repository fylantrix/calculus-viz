use std::process::Command;

const CFILE: &str = "plotters-doc-data/0.png";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("hello world");
    // Command::new("xdg-open")
    //     .arg("plotters-doc-data/0.png")
    //     .spawn()
    //     .expect("image failed to open");
    // println!("opened {}", CFILE);


    Ok(())
}
