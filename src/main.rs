extern crate notify;
extern crate term;

use notify::{RecommendedWatcher, Error, Watcher};
use std::sync::mpsc::channel;

use std::thread;
use std::string::String;
use std::path::Path;
use std::process::Command;

fn main() {
    let mut t_out = term::stdout().unwrap();
    t_out.fg(term::color::GREEN);
    println!("\nNow watching ./src");
    t_out.reset();


    let (tx, rx) = channel();
    let w: Result<RecommendedWatcher, Error> = Watcher::new(tx);


    match w {

        Ok(mut watcher) =>  {
            match watcher.watch(&Path::new("./src/")) {
                Ok(_) => {},
                Err(e) => {
                    println!("Error creating watcher: {:?}", e);
                    return
                },
            }

            loop {
                thread::sleep_ms(5000);

                match rx.recv() {
                    Ok(e) => handle(e),
                    Err(e) => println!("{:?}", e),
                }
            }
        },
        Err(_) => println!("Couldn't generate directory watcher")
    }
}

fn handle(e: notify::Event) {
    match e.op {
        Ok(_) => {
            let mut t_out = term::stdout().unwrap();

            let output = Command::new("cargo")
                                    .arg("test")
                                    .output()
                                    .unwrap_or_else(|e| { panic!("Failed to execute cargo: {}", e)});
                                    
            println!("{}", String::from_utf8_lossy(&output.stdout));
            println!("{}", String::from_utf8_lossy(&output.stderr));
            if (output.status.success()) {
                t_out.fg(term::color::GREEN);
                println!("\n\n\nTEST SUCCESSFUL\n\n\n");
            } else {
                t_out.fg(term::color::RED);
                println!("\n\n\nTEST FAILED\n\n\n");
            }
            t_out.reset();
        },
        Err(e) => println!("{:?}", e),
    }
}
