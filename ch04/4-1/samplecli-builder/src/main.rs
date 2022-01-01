use clap::{arg, App};

fn main() {
    let matches = App::new("My RPN program")
        .version("1.0.0")
        .author("Your name")
        .about("Super awesome sample RPN calculator")
        .arg(arg!([FILE] "Formulas written in RPN").required(false))
        .arg(arg!(-v --verbose ... "Sets the level of verbosity").required(false))
        .get_matches();

    match matches.value_of("FILE") {
        Some(file) => println!("File specified: {}", file),
        None => println!("No file specified."),
    }
    let verbose = matches.is_present("verbose");
    println!("Is verbosity specified?: {}", verbose);
}
