use std::env::current_dir;
use std::io::ErrorKind;
use std::os::unix;
use std::os::unix::fs::PermissionsExt;
use std::str::FromStr;
use std::{env, fs};

use clap::{command, Arg, ColorChoice, ValueHint};
use owo_colors::{colors, OwoColorize};

/*
TODO clap_autocomplete
TODO mode
TODO Localisation
TODO CTX
TODO Windows compatibility check
*/

fn main() {
	let command = clap_autocomplete::add_subcommand(command!())
		.name("mkd")
		.author("LeSnake <dev.lesnake@posteo.de>")
		.about("Modern replacement for mkdir written in rust")
		.arg_required_else_help(true)
		.color(ColorChoice::Auto)
		.arg(
			Arg::new("dirs")
				.value_name("DIRECTORY")
				.help("Path to the directory")
				.multiple_occurrences(true)
				.value_hint(ValueHint::DirPath),
		)
		.arg(
			Arg::new("parents")
				.short('p')
				.long("parents")
				.help("Create Parent directories if they don't exists"),
		)
		.arg(
			Arg::new("no-error")
				.short('e')
				.long("no-error")
				.help("Ignore error if folder exists"),
		)
		.arg(
			Arg::new("verbose")
				.short('v')
				.long("verbose")
				.help("Enable output on Success"),
		)
		.arg(
			Arg::new("mode")
				.short('m')
				.long("mode")
				.takes_value(true)
				.help("Permission of new folders"),
		);
	let command_copy = command.clone();
	let matches = command.get_matches();

	if let Some(result) = clap_autocomplete::test_subcommand(&matches, command_copy) {
		if let Err(err) = result {
			eprintln!("Insufficient permissions: {err}");
			std::process::exit(1);
		} else {
			std::process::exit(0);
		}
	}

	let dirs = matches.values_of("dirs").expect("No name for folder");
	let parents = matches.is_present("parents");
	let no_error = matches.is_present("no-error");
	let verbose = matches.is_present("verbose");
	let mode = matches.value_of("mode");

	macro_rules! verbose_msg {
		( $msg: expr) => {
			if verbose {
				println!("{}", $msg)
			}
		};
	}

	for dir in dirs {
		/*
		Generate Path
		 */
		let path = if dir.to_string().starts_with('/') {
			dir.to_string()
		} else {
			format!(
				"{}/{}",
				current_dir().expect("Couldn't get path").to_string_lossy(),
				dir
			)
		};
		println!("{}", path);

		/*
		Create folder
		 */
		if let Err(e) = if parents {
			fs::create_dir_all(dir)
		} else {
			fs::create_dir(dir)
		} {
			/*
			Error Handling
			 */
			print!(
				"{}: {}",
				dir.fg::<colors::BrightYellow>(),
				match e.kind() {
					ErrorKind::PermissionDenied => format!(
						"{}{}",
						"Permission denied\n\n".fg::<colors::Red>(),
						"Potential Fixes:\n\
						- Change folder owner or folder permissions\n\
						- Run with Sudo\n"
							.fg::<colors::BrightBlack>()
					),
					ErrorKind::AlreadyExists =>
						if !no_error {
							format!("{}", "Folder already exists\n".fg::<colors::Red>())
						} else {
							"".to_string()
						},

					ErrorKind::NotFound => format!(
						"{}{}",
						"Parent folder doesn't exist\n\n".fg::<colors::Red>(),
						"Tip: Use -p or --parents to automatically create parent folders.\n"
							.fg::<colors::BrightBlack>()
					),
					_ => format!("Unknown error: {:?}\n", e),
				}
			);
		} else {
			verbose_msg!(
				format!("{}: Folder created successfully", dir).fg::<colors::BrightYellow>()
			)
		}

		/*
		Set file permission
		 */
		if !mode.is_none() {
			println!("{}", &mode.unwrap());
			fs::set_permissions(
				path,
				fs::Permissions::from_mode(
					0ou32::from_str(&mode.unwrap()).expect("Error Parsing Mode"),
				),
			)
			.expect("Couldn't set permission");
		}
	}
}
