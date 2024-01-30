use crate::prelude::*;
use std::env;
use std::process::Command;

pub fn ast_compile_single(contract: &str) -> bool {
    //generate sol ast from config file
    let project = Project::builder().build().unwrap();
    let output = project.compile_files(vec![contract]).unwrap();
    let ast = output
        .output()
        .sources
        .0
        .values()
        .next()
        .iter_mut()
        .next()
        .as_mut()
        .unwrap()
        .get(0) 
        .unwrap()
        .source_file
        .ast
        .clone()
        .unwrap();
    fs::write("src", serde_json::to_string(&ast).unwrap()).expect("failed to generate ast");
    true
}


pub fn ast_sh_command() {
    //get the contract file name from the command-line argument
    let args: Vec<String> = env::args().collect();
    let contract = &args[1];
    ast_compile_single(contract);
}

pub fn checker() -> bool{
    let check = ast_compile_single("../src/gen_ast/example.sol");
    check
}

#[cfg(test)]
mod tests {
    use crate::gen_ast::ast::checker;

    use super::ast_compile_single;

    #[test]
    fn test_ast_compile_single() {
        let check = checker();
        assert!(check);
    }

}