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
            let _ = watcher.watch(&Path::new("./src/"));

            loop {
                thread::sleep_ms(5000);

                match rx.recv() {
                    Ok(e) => handle(e),
                    Err(_) => (),
                }
            }
        },
        Err(_) => println!("Couldn't generate directory watcher")
    }
}


fn handle(e: notify::Event) {
    match e.op {
        Ok(_) => {
            let output = Command::new("cargo")
                                    .arg("test")
                                    .output()
                                    .unwrap_or_else(|e| { panic!("Failed to execute cargo: {}", e)});
                                    
            println!("{}", String::from_utf8_lossy(&output.stdout));
        },
        Err(e) => println!("{:?}", e),
    }
}
