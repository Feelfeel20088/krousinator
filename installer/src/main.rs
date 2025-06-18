use std::{process::Command, path::PathBuf};
use ptrace_inject::Injector;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("doing the thing");
    let library = PathBuf::from("/home/felix/projects/krousinator/target/debug/libkrousinator.so");
    
    
    // Spawn a new process and inject the library into it.
    let mut target: Command = Command::new("/home/felix/projects/krousinator/target/debug/dummy"); 
    
    target.arg("1000");

    Injector::spawn(target)?.inject(&library)?;


    Ok(())


}




