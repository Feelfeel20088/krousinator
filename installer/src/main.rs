use std::process::Command;
use std::path::PathBuf;
use ptrace_inject::Injector;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting injection...");


    let library = PathBuf::from("/home/felix/projects/krousinator/target/debug/libkrousinator.so");

  
    let mut target = Command::new("/home/felix/projects/krousinator/target/debug/dummy");
    target.arg("1000");

 
    let mut injector = Injector::spawn(target)?;


    injector.inject(&library)?;

    println!("Injection complete.");
    Ok(())
}
