use clap::{crate_authors, crate_name, crate_version, App, Arg, SubCommand};

pub fn app_config<'a>() -> App<'a, 'a> {
    // Initialize Application
    let app = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about("Manage User-Mode Linux by YAML");

    let file_arg = Arg::with_name("file")
        .takes_value(true)
        .required(false)
        .short("f");

    // SubCommands {{
    let run_command = SubCommand::with_name("run")
        .about("apply manifests")
        .arg(&file_arg);

    app.subcommand(run_command)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn run_f_should_be_success() {
        let args = vec!["umlctl", "run", "-f", "example.yaml"];

        app_config()
            .get_matches_from_safe(&args)
            .expect("get_matches_from_safe: ");
    }

    #[test]
    fn run_f_should_be_fail_if_file_is_not_specified() {
        let args = vec!["umlctl", "run", "-f"];

        app_config()
            .get_matches_from_safe(&args)
            .expect_err("get_matches_from_safe: ");
    }
}
