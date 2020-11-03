//------------------------------------------------------------------------------
// Project: jrnlvw
// File name: opt.rs
// File Description: The Parser options definitions
// License: MIT
//------------------------------------------------------------------------------

use std::path::Path;

//------------------------------------------------------------------------------
// Command line interface flags
//------------------------------------------------------------------------------

pub const LOGFILE: &str = "logfile";
pub const LIST_BOOTS_FLAG: &str = "list-boots";
pub const LOG_LEVEL: &str = "priority";
pub const BOOT_FILTER: &str = "boot";
pub const UNIT_FILTER: &str = "unit";
pub const KERNEL_FLAG: &str = "kernel";
pub const NUM_OF_ENTRIES: &str = "number";
//Todo: add time arguments, from/to point in time

//------------------------------------------------------------------------------
// File Command line options/argument struct type
//------------------------------------------------------------------------------

#[derive(Debug, Default, Clone)]
pub struct CliOptions {
   logfile: String,
   list_boots: bool,
   log_level: u32,
   boot_filter: Vec<String>,
   unit_filter: Vec<String>,
   kernel_flag: bool,
   num_of_entries: u32,
}

//------------------------------------------------------------------------------
// FilePermission associated methods
//------------------------------------------------------------------------------

impl CliOptions {

   // Getters for cli options members

   // Get path to logfile
   pub fn logfile_path(&self) -> &Path
   {
      return Path::new(&self.logfile);
   }

   pub fn logfile_name(&self) -> &String
   {
      return &self.logfile;
   }

   pub fn list_boots(&self) -> bool
   {
      return self.list_boots;
   }

   pub fn log_level(&self) -> u32
   {
      return self.log_level;
   }

   pub fn boot_filter(&self) -> &Vec<String>
   {
      return &self.boot_filter;
   }

   pub fn unit_filter(&self) -> &Vec<String>
   {
      return &self.unit_filter;
   }

   pub fn kernel_flag(&self) -> bool
   {
      return self.kernel_flag;
   }

   pub fn num_of_entries(&self) -> u32
   {
      return self.num_of_entries;
   }
}

//------------------------------------------------------------------------------
// FilePermission struct associated functions
//------------------------------------------------------------------------------

impl CliOptions {

   // parse and set argument values from matches
   pub fn from_matches(matches: &clap::ArgMatches) -> Result<CliOptions, failure::Error>
   {
      // return object
      let mut cli_opt = CliOptions::default();

      cli_opt.logfile = matches.value_of(LOGFILE).unwrap().to_string();

      // set list-boots flag, if provided
      if matches.is_present(LIST_BOOTS_FLAG) {
         cli_opt.list_boots = true;
      } else {
         cli_opt.list_boots = false;
      }

      // set log level, if provided
      if matches.is_present(LOG_LEVEL) {
         let level = matches.value_of(LOG_LEVEL).unwrap();
         let level = level.parse::<u32>()?;

         // verify log level range
         if level > 7 {
            eprintln!("Invalid log level: {}, default level DEBUG (7) will be used", level);
            let level = 7;
            cli_opt.log_level = level;
         } else {
            cli_opt.log_level = level;
         }
      } else {
         cli_opt.log_level = 7;
      }

      // set boot filter, if provided
      if matches.is_present(BOOT_FILTER) {
         let boot_ids = matches.values_of(BOOT_FILTER).unwrap();
         for id in boot_ids {
            cli_opt.boot_filter.push(id.to_string());
         }
      }

      // set (systemd) unit filter, if provided
      if matches.is_present(UNIT_FILTER) {
         let units = matches.values_of(UNIT_FILTER).unwrap();
         for unit in units {
            // "raw" unit name, if .service provided by user
            cli_opt.unit_filter.push(unit.to_string());

            if unit.contains(".service") {
               continue;
            }
            // add service variant of unit name
            let unit_service = unit.to_string() + &".service";
            cli_opt.unit_filter.push(unit_service.to_string());
         }
      }

      if matches.is_present(KERNEL_FLAG) {
         cli_opt.kernel_flag = true;
      } else {
         cli_opt.kernel_flag = false;
      }

      if matches.is_present(NUM_OF_ENTRIES) {
         let num = matches.value_of(NUM_OF_ENTRIES).unwrap();
         let num = num.parse::<u32>()?;
         cli_opt.num_of_entries = num;
      }

      return Ok(cli_opt);
   }
}
