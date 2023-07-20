use clap::{arg, Command};

const ABOUT: &str = "\x1b[94mnextup\x1b[0m keeps your barrier to starting project sessions \
                    under control by tracking the next action to take on a maximum of 3 \
                    active projects.";

const HELP: &str = "For best results, update a nextup when switching away from a project to \
     record one achievable action to complete when you next pick it up or simply capture the \
     state you are leaving things in.";

/// Define all commands and args for the nextup app
///
/// nextup
/// nextup <a, b, c>
/// nextup set <a, b, c> <title>
/// nextup <a, b, c> <nextup>
/// /// nextup reset
pub fn commands() -> Command {
    Command::new("nextup")
        .about(ABOUT)
        .after_help(HELP)
        .arg(arg!(project: [PROJECT] "project to view").value_parser(["a", "b", "c"]))
        .arg(
            arg!(nextup: [NEXTUP] "(optional) update what's nextup for project")
                .value_parser(clap::builder::NonEmptyStringValueParser::new()),
        )
        .subcommand(
            Command::new("reset")
                .short_flag('r')
                .about("wipes all set projects and nextups"),
        )
        .subcommand(
            Command::new("set")
                .about("set one of the three project slots")
                .arg_required_else_help(true)
                .arg(
                    arg!(project: [PROJECT] "project to set")
                        .required(true)
                        .value_parser(["a", "b", "c"]),
                )
                .arg(
                    arg!(title: [DESC] "short project title or description")
                        .required(true)
                        .value_parser(clap::builder::NonEmptyStringValueParser::new()),
                ),
        )
}
