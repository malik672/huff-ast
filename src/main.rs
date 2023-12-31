use crate::prelude::*;
mod gen_ast;
mod prelude;

fn main() {
    // Get command-line arguments passed by the Bash script
    let args: Vec<String> = env::args().collect();

    // Check if an argument is provided
    if args.len() > 1 {
        // The first argument is the program name, and the second is the parameter
        let user_param = &args[1];
        gen_ast::ast::ast(&user_param);
        println!("User parameter: {}", user_param);
    } else {
        println!("No user parameter provided.");
    }
}
