use crate::prelude::*;

pub fn ast(contract: &str) {
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
}
