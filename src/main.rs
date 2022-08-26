extern crate clap;

use clap::{Arg, App};


fn main() {
    //命令行简介 
    let matches = App::new("kt")
      .version("0.1.0")
      .author("Gao Song")
      .about("Command Test")
      .arg(Arg::with_name("FILE")
            .help("File to print.")
            .empty_values(false)
        )
      .get_matches();

    //参数匹配
    if let Some(file) = matches.value_of("FILE") {
        println!("Value for file argument: {}", file);
    }

}
