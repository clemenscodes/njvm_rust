use njvm::{
    help, kill,
    utils::{fatal_error::fatal_error, unknown_arg::unknown_arg},
    version, NinjaVM,
};

fn main() {
    let mut vm = NinjaVM::default();
    match std::env::args().len() {
        1 => fatal_error("Error: no code file specified"),
        2 => {
            let arg = &std::env::args().nth(1).unwrap();
            match arg as &str {
                "--help" => help(),
                "--version" => version(),
                "--debug" => fatal_error("Error: no code file specified"),
                _ => {
                    if arg.starts_with('-') {
                        unknown_arg(arg)
                    }
                    vm.execute_binary(arg)
                }
            }
        }
        3 => {
            let first_arg = &std::env::args().nth(1).unwrap();
            let second_arg = &std::env::args().nth(2).unwrap();
            match first_arg as &str {
                "--help" => help(),
                "--version" => version(),
                "--debug" => match second_arg as &str {
                    "--help" => help(),
                    "--version" => version(),
                    "--debug" => fatal_error("Error: no code file specified"),
                    _ => {
                        if second_arg.starts_with('-') {
                            unknown_arg(second_arg)
                        }
                        vm.debug(second_arg)
                    }
                },
                _ => match second_arg as &str {
                    "--help" => help(),
                    "--version" => version(),
                    "--debug" => vm.debug(first_arg),
                    _ => {
                        if second_arg.starts_with('-') {
                            unknown_arg(second_arg)
                        }
                        fatal_error("Error: more than one code file specified")
                    }
                },
            }
        }
        _ => kill(),
    }
}
