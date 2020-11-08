//------------------------------------------------------------------------------
// Project: jrnlvw
// File name: cli.rs
// File Description: The command line interface definitions
// License: MIT
//------------------------------------------------------------------------------

use clap::{App, AppSettings, Arg};

use crate::opt;
//------------------------------------------------------------------------------

pub fn create_cli() -> App<'static, 'static> {
    App::new("jrnlvw - journal viewer")
        .version(crate_version!())
        .author(crate_authors!("\n"))
        .about(crate_description!())
        .setting(AppSettings::ColorAlways)
        .arg(
            Arg::with_name(opt::LOGFILE)
                .help("The journal json logfile to view")
                .required(true),
        )
        .arg(
            Arg::with_name(opt::LIST_BOOTS_FLAG)
                .help("List all boots from provided logfile")
                .required(false)
                .long(opt::LIST_BOOTS_FLAG)
                .short("l")
                .takes_value(false),
        )
        .arg(
            Arg::with_name(opt::KERNEL_FLAG)
                .help("Only print log entry originating from the kernel")
                .required(false)
                .long(opt::KERNEL_FLAG)
                .short("k")
                .takes_value(false),
        )
        .arg(
            Arg::with_name(opt::LOG_LEVEL)
                .help("Set entry log level to print, default 7 = debug")
                .required(false)
                .long(opt::LOG_LEVEL)
                .short("p")
                .takes_value(true),
        )
        .arg(
            Arg::with_name(opt::UNIT_FILTER)
                .help("systemd unit(s) to print")
                .required(false)
                .long(opt::UNIT_FILTER)
                .short("u")
                .takes_value(true)
                .multiple(true),
        )
        .arg(
            Arg::with_name(opt::NUM_OF_ENTRIES)
                .help("Max amount of log entries to print for each boot (<NUMBER> > 0)")
                .required(false)
                .long(opt::NUM_OF_ENTRIES)
                .short("n")
                .takes_value(true),
        )
        .arg(
            Arg::with_name(opt::BOOT_FILTER)
                .help("Specify a boot(s) to show ")
                .required(false)
                .long(opt::BOOT_FILTER)
                .short("b")
                .takes_value(true)
                .multiple(true),
        )
        .arg(
            Arg::with_name(opt::TIME_FROM)
            .help(
                "Specify a point in time (UTC hour:minute:second) as a start point for displaying log entries.")
            .required(false)
            .long(opt::TIME_FROM)
            .short("t")
            .takes_value(true)
        )
        .arg(
            Arg::with_name(opt::TIME_TO)
            .help("Specify a point in time (UTC hour:minute:second) as a stop point for displaying log entries")
            .required(false)
            .long(opt::TIME_TO)
            .short("T")
            .takes_value(true)
        )
        .arg (
            Arg::with_name(opt::DATE_FROM)
            .help("Specify a day in time (UTC year:month:day) as a start point for displaying log entries")
            .required(false)
            .long(opt::DATE_FROM)
            .short("d")
            .takes_value(true)
        )
        .arg (
            Arg::with_name(opt::DATE_TO)
            .help("Specify a day in time (UTC year:month:day) as a stop point for displaying log entries")
            .required(false)
            .long(opt::DATE_TO)
            .short("D")
            .takes_value(true)
        )
    //Todo, add arg for creating a output file

    // Todo: more filters
}
