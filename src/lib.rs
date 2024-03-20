use std::fs::read_dir;

use proc_macro::TokenStream;

#[proc_macro]
pub fn project_euler(_: TokenStream) -> TokenStream {
    let mut problems: Vec<String> = read_dir("src")
        .expect("Cannot read src dir")
        .map(|i| i.unwrap())
        .filter(|i| i.file_name().to_string_lossy().starts_with('p'))
        .map(|i| i.file_name().into_string().unwrap())
        .collect();
    problems.sort();
    let mut da_code = String::new();
    da_code.push_str("use std::time::{SystemTime, UNIX_EPOCH};");

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

        let m = spl.next().unwrap();
        da_code.push_str("let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();\n");
        da_code.push_str(format!("let result = {}::solution();\n", m).as_str());
        da_code.push_str("let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();\n");
        da_code.push_str(
            format!(
                "println!(\"{}: {{}}, time: {{:?}}\", result, end - start);",
                m
            )
            .as_str(),
        );
    }
    da_code.push_str("}\n}");
    da_code.push_str("else {\n");
    let mut last = problems.last().unwrap().split(".");
    let m = last.next().unwrap();
    da_code.push_str("let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();\n");
    da_code.push_str(format!("let result = {}::solution();\n", m).as_str());
    da_code.push_str("let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();\n");
    da_code.push_str(
        format!(
            "println!(\"{}: {{}}, time: {{:?}}\", result, end - start);",
            m
        )
        .as_str(),
    );
    da_code.push('}');

    da_code.push_str("}");
    da_code.parse().unwrap()
}
