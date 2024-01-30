use std::fs::read_dir;

use proc_macro::TokenStream;

#[proc_macro]
pub fn project_euler(_: TokenStream) -> TokenStream {
    let problems: Vec<String> = read_dir("src")
        .expect("Cannot read src dir")
        .map(|i| i.unwrap())
        .filter(|i| i.file_name().to_string_lossy().starts_with('p'))
        .map(|i| i.file_name().into_string().unwrap())
        .collect();
    let mut da_code = String::new();

    for i in problems.iter() {
        let mut spl = i.split('.');
        da_code.push_str(format!("mod {};\n", spl.next().unwrap()).as_str());
    }

    da_code.push_str("fn main() {\n");
    let run_all = "if let Some(i) = std::env::args().nth(1) {
    if i == \"--all\" {\n";
    da_code.push_str(run_all);
    for p in problems.iter() {
        let mut spl = p.split('.');
        da_code.push_str(format!("{}::solution();\n", spl.next().unwrap()).as_str())
    }
    da_code.push_str("}\n}");
    da_code.push_str("else {\n");
    let mut last = problems.last().unwrap().split(".");
    da_code.push_str(format!("{}::solution();\n", last.next().unwrap()).as_str());
    da_code.push('}');

    da_code.push_str("}");
    da_code.parse().unwrap()
}
