use crate::cli::CreateArgs;
use crate::output;
use crate::prefix::detect;
use crate::scaffold::{self, Options};
use crate::{Error, Result};

pub fn run(args: CreateArgs) -> Result<()> {
    let (prefix, project_name) = detect(&args.name)?;

    let lang = if args.lang.is_empty() {
        prefix.default_lang.to_string()
    } else {
        args.lang.clone()
    };

    if !prefix.has_lang(&lang) {
        return Err(Error::UnsupportedLanguage {
            lang,
            prefix: prefix.name.to_string(),
            available: prefix.langs.join(" "),
        });
    }

    let options = Options {
        full_name: args.name.clone(),
        prefix: prefix.name.to_string(),
        project_name: project_name.to_string(),
        lang,
        author: args.author,
        module_path: args.module,
        license: args.license,
        output_dir: args.output,
        description: prefix.description.to_string(),
    };

    scaffold::generate(prefix, &options)?;
    output::print_success(&args.name, &options.output_dir);

    Ok(())
}
