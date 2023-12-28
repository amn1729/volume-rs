use clap::{value_parser, Arg, ArgAction, Command};

pub fn get_args() -> Command {
    Command::new("Control pulseaudio volume of default device")
        .author("Anhsirk0, krishna404@yandex.com")
        .version("1.0.0")
        .arg(
            Arg::new("set")
                .long("set")
                .short('s')
                .help("Set current volume")
                .value_parser(value_parser!(usize)),
        )
        .arg(
            Arg::new("inc")
                .long("inc")
                .short('i')
                .help("Increase current volume")
                .value_parser(value_parser!(usize)),
        )
        .arg(
            Arg::new("dec")
                .long("dec")
                .short('d')
                .help("Decrease current volume")
                .value_parser(value_parser!(usize)),
        )
        .arg(
            Arg::new("get")
                .long("get")
                .short('g')
                .action(ArgAction::SetTrue)
                .help("Get current volume"),
        )
        .arg(
            Arg::new("mute")
                .long("mute")
                .short('m')
                .action(ArgAction::SetTrue)
                .help("Toggle mute"),
        )
}
