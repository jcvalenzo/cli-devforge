use crate::prefix::Prefix;

pub fn print_success(name: &str, output_dir: &str) {
    println!();
    println!("Project {name:?} created successfully in ./{output_dir}/{name}");
    println!();
    println!("Next steps:");
    println!("  cd {name}");
    println!("  git init");
    println!("  git add .");
    println!("  git commit -m \"Initial commit\"");
    println!();
    println!("Push to GitHub:");
    println!("  gh repo create {name} --public --source=. --remote=origin --push");
    println!();
    println!("Or configure remote manually:");
    println!("  git remote add origin git@github.com:YOUR_GITHUB_USERNAME/{name}.git");
    println!();
}

pub fn print_list(prefixes: &[Prefix]) {
    let mut rows: Vec<[String; 3]> = Vec::with_capacity(prefixes.len() + 2);
    rows.push([
        "PREFIX".to_string(),
        "DESCRIPTION".to_string(),
        "LANGUAGES".to_string(),
    ]);
    rows.push([
        "------".to_string(),
        "-----------".to_string(),
        "---------".to_string(),
    ]);

    for prefix in prefixes {
        rows.push([
            format!("{}-", prefix.name),
            prefix.description.to_string(),
            prefix.langs.join(", "),
        ]);
    }

    let widths = (0..3)
        .map(|column| {
            rows.iter()
                .map(|row| row[column].chars().count())
                .max()
                .unwrap_or(0)
        })
        .collect::<Vec<_>>();

    for row in rows {
        let mut line = String::new();
        for (column, cell) in row.iter().enumerate() {
            line.push_str(cell);
            if column + 1 < row.len() {
                let padding = widths[column] - cell.chars().count() + 2;
                line.push_str(&" ".repeat(padding));
            }
        }
        println!("{line}");
    }
}
