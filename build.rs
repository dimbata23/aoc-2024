use std::fs::{self, OpenOptions};
use std::io::{self, Write};

fn main() -> io::Result<()> {
    let days = get_days()?;
    generate_days_rs(&days)?;
    generate_toml(&days)?;
    Ok(())
}

fn get_days() -> io::Result<Vec<String>> {
    Ok(fs::read_dir("./")?
        .filter_map(Result::ok)
        .filter(|entry| entry.file_name().to_str().unwrap().starts_with("day"))
        .map(|entry| entry.file_name().to_string_lossy().into_owned())
        .collect::<Vec<String>>())
}

fn generate_days_rs(days: &[String]) -> io::Result<()> {
    fs::copy("./src/reg_days_template", "./src/register_days.rs")?;
    let mut file = OpenOptions::new()
        .append(true)
        .open("./src/register_days.rs")?;

    let mut out = String::new();
    days.iter().for_each(|day| {
        out.push_str(&format!("    use {};\n", day));
        out.push_str(&format!("    days.insert(\"{day}\", {day}::run);\n"));
    });

    writeln!(file, "{out}")?;
    writeln!(file, "    days\n}}")?;

    Ok(())
}

fn generate_toml(days: &[String]) -> io::Result<()> {
    fs::copy("Cargo_base.toml", "Cargo.toml")?;
    let mut file = OpenOptions::new().append(true).open("Cargo.toml")?;

    let days_as_deps = days
        .iter()
        .map(|day| format!("{day} = {{ path = \"{day}\" }}"))
        .collect::<Vec<_>>()
        .join("\n");
    writeln!(file, "{days_as_deps}\n")?;

    let members = format!("[workspace]\nmembers = {:?}", days);
    writeln!(file, "{members}\n")?;

    Ok(())
}
