
// the art of me 






fn main() {

    //

    //warn the user this program is dangerous that it will melt their computer it art 

    //check app runing in terminal
    if std::env::var("TERM").is_err() {
        eprintln!("This program is meant to be run in a terminal. Exiting.");
        std::process::exit(1);
    }
        

    println!("Hello, world!");
}
