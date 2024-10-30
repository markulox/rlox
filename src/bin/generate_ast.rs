use std::{
    collections::VecDeque, env, error::Error, fmt::format, fs, io::Write, os::unix::ffi::OsStrExt,
    path::Path, process::ExitCode,
};

use anyhow::Ok;

fn main() -> ExitCode {
    let mut args: VecDeque<String> = env::args().collect();
    args.pop_front();
    if args.len() != 1 {
        println!("Usage: generate_ast <output directory>");
        ExitCode::from(64)
    } else {
        let output_dir_str = args.pop_back().unwrap_or(String::from("./"));
        let output_dir = Path::new(&output_dir_str);
        if let Err(e) = define_ast(
            output_dir,
            "Expr",
            VecDeque::from([
                "Binary     :   Expr left, Token operator, Expr right",
                "Grouping   :   Expr expression",
                "Literal    :   Object value",
                "Unary      :   Token operator, Expr right",
            ]),
        ) {
            println!("<X> Error: {e}");
            ExitCode::FAILURE
        } else {
            ExitCode::SUCCESS
        }
    }
}

fn define_ast(
    output_dir: &Path,
    base_name: &str,
    types: VecDeque<&str>,
) -> Result<(), anyhow::Error> {
    // let file_name = base_name.to_ascii_lowercase();
    let binding = output_dir.join(Path::new(&format!("mod.rs")));
    let full_path = binding.as_path();
    println!(
        "<i> Generating AST file: {}",
        full_path.as_os_str().to_str().unwrap_or_default()
    );
    fs::create_dir_all(output_dir)?;
    let mut f = fs::File::create(full_path)?;
    f.write(b"use std::any::Any;\n")?;
    f.write(b"use super::scanner::token::Token;\n\n")?;
    f.write(b"type Object = Box<dyn Any>;\n")?;
    f.write(format!("trait {base_name} {{}}\n").as_bytes())?;
    for e_type in types {
        let splitted_type: Vec<&str> = e_type.split(':').collect();
        let struct_name = splitted_type.get(0).unwrap().trim();
        let fields = splitted_type.get(1).unwrap().trim();
        define_type(&mut f, base_name, struct_name, fields)?;
    }
    f.flush()?;
    Ok(())
}

fn define_type(
    f: &mut fs::File,
    base_name: &str,
    struct_name: &str,
    fields: &str,
) -> Result<(), anyhow::Error> {
    // SECTION: Struct defining
    f.write(format!("struct {struct_name} {{\n").as_bytes())?;
    let mut processed_type_var_pair: Vec<(String,String)> = Vec::new();
    let splitted_field: Vec<&str> = fields.split(',').into_iter().map(|e| e.trim()).collect();
    for e_spl_field in splitted_field {
        let type_var_pair: Vec<&str> = e_spl_field.split_whitespace().collect();
        let type_str = if type_var_pair.get(0).unwrap().trim() == base_name {
            &format!("Box<dyn {base_name}>")
        } else {
            type_var_pair.get(0).unwrap().trim()
        };
        let var_name = type_var_pair.get(1).unwrap().trim();
        processed_type_var_pair.push((type_str.to_string(), var_name.to_string()));
        f.write(format!("\tpub {var_name}: {type_str},\n").as_bytes())?;
    }
    f.write(b"}\n")?;
    // END SECTION
    // SECTION: Add trait
    f.write(format!("impl {base_name} for {struct_name} {{}}\n").as_bytes())?;
    // END SECTION
    // SECTION: Defining constructor
    f.write(format!("impl {struct_name} {{\n").as_bytes())?;
    f.write(format!("\tfn new(").as_bytes())?;
    for (type_str, var_name) in &processed_type_var_pair {
        f.write(format!("{var_name}: {type_str}, ").as_bytes())?;
    }
    f.write(format!(") -> Box<Self> {{\n\t\tBox::new({struct_name} {{").as_bytes())?;
    let var_list = processed_type_var_pair.iter().fold(String::new(), 
    |mut accu, e_pair| {
        accu.push_str(format!("{},",e_pair.1).as_str());
        accu
    });
    f.write(format!("{var_list} }})\n\t}}\n}}\n\n").as_bytes())?;
    // END SECTION
    Ok(())
}
