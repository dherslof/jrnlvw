//------------------------------------------------------------------------------
// Project: jrnlvw
// File name: parser.rs
// File Description: The systemd journal logfile parser functionality
// License: MIT
//------------------------------------------------------------------------------
use std::fs::File;
use std::collections::HashMap;
use std::io::{BufReader, BufRead};

use chrono::prelude::*;
use chrono::UTC;

use serde::{Deserialize, Serialize};

use crate::opt;

//------------------------------------------------------------------------------
// systemd journal log entry struct type
//------------------------------------------------------------------------------

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize)]
struct LogEntry {
   __CURSOR: Option<String>,
    __REALTIME_TIMESTAMP: Option<String>,
    __MONOTONIC_TIMESTAMP: Option<String>,
    _BOOT_ID: Option<String>,
    _TRANSPORT: Option<String>,
    SYSLOG_FACILITY: Option<String>,
    _UID: Option<String>,
    _GID: Option<String>,
    _MACHINE_ID: Option<String>,
    SYSLOG_IDENTIFIER: Option<String>,
    _PID: Option<String>,
    _CMDLINE: Option<String>,
    _SYSTEMD_CGROUP: Option<String>,
    _SYSTEMD_UNIT: Option<String>,
    MESSAGE: Option<String>,
    _HOSTNAME: Option<String>,
    PRIORITY: Option<String>,
    CODE_FILE: Option<String>,
    CODE_LINE: Option<String>,
    CODE_FUNCTION: Option<String>,
    ERRNO: Option<String>,
    UNIT: Option<String>,

    // DOC:
    // https://www.freedesktop.org/software/systemd/man/systemd.journal-fields.html
}

//------------------------------------------------------------------------------
// Formated log entry type
//------------------------------------------------------------------------------
struct FormattedLogEntry
{
   sequence_number: String,
   timestamp: String,
   loglevel: String,
   unit_name: String,
   message: String,
}

//------------------------------------------------------------------------------
// Parsed log file struct type
//------------------------------------------------------------------------------

pub struct ParsedLogfile
{
   filename: String,
   log_entries: Vec<LogEntry>,
   total_entries: usize,
   parse_opt: opt::CliOptions,
   boot_ids: Vec<String>,
}

//------------------------------------------------------------------------------
// ParsedLogfile struct associated functions
//------------------------------------------------------------------------------

impl ParsedLogfile {

   // Parse log file and return a new ParsedLogfile
   pub fn new (parse_options: &opt::CliOptions) -> Result<ParsedLogfile, failure::Error>
   {
      //
      let mut entries: Vec<LogEntry> = Vec::new();
      let mut ids: Vec<String> = Vec::new();

      #[allow(non_snake_case)]
      let NOT_AVAILABLE = String::from("N/A");
      //Dev
      println!("DEV: starting to parsing file");

      // open and read file json formatted systemd journal file
      let file = File::open(parse_options.logfile_path())?;
      let buf_reader = BufReader::new(file);

      for line in buf_reader.lines() {
         if line.is_err() {
            eprintln!("Failed to read line, ignoring");
            continue;
         }

         // get result as str and convert from json to LogEntry
         let line_string = line.unwrap();
         let line_str = line_string.as_str();
         let entry_result = serde_json::from_str(line_str);
         if entry_result.is_err() {
            eprintln!("Illformated line: {:?} - Ignoring entry!", entry_result.err().unwrap());
            continue;
         }
         let entry: LogEntry = entry_result.unwrap();
         entries.push(entry);
      }

      println!("DEV-PRINT: size of entry vector {}", entries.len());
      let number = entries.len();

      match entries[0]._BOOT_ID {
         Some(ref id) => {ids.push(id.clone()); },
         _ => eprintln!("Unable to get boot ID"),
      }

      // Get all boot IDs from entry list
      for e in &entries {

         let  boot_id: String = match &e._BOOT_ID {
            Some(ref id) => id.clone(),
            None => {eprintln!("Unable to get BOOT_ID from entry"); NOT_AVAILABLE.clone()}
         };

         if !ids.contains(&boot_id) {
            ids.push(boot_id.clone());
         }
      }
      return Ok(Self{
         filename: parse_options.logfile_name().clone(), log_entries: entries,
         total_entries: number, parse_opt: parse_options.clone(), boot_ids: ids })
   }

}

//------------------------------------------------------------------------------
// FilePermission associated methods
//------------------------------------------------------------------------------

impl ParsedLogfile {

   // Public function to print the log entries
   pub fn print(self)
   {
      // Just for separation of the output in terminal
      println!(" ");

      // Print boot IDs list if flag is set
      if self.parse_opt.list_boots() {
         self.print_boot_list();
         return;
      }

      //Sort and format Entries
      let format_entries_result = self.format_entry_list();

      // Check result
      if format_entries_result.is_err() {
         eprintln!(
            "Error accord while formatting log entries [ {:?} ], unable to display logs",
            format_entries_result.err().unwrap());
         return;
      }

      let formatted_entries = format_entries_result.unwrap();
      for (boot, entries) in formatted_entries.iter()
      {
         // print current boot 'header'
         self.print_boot(&boot, self.total_entries);
         self.print_column_header();

         // Check if entry limit filter is used:
         let mut e_counter: u32 = 1;
         let e_nr_limit = self.parse_opt.num_of_entries();
         for entry in entries
         {
            if e_nr_limit == 0 {
               self.print_entry(entry);
            } else if e_counter <= e_nr_limit && e_nr_limit > 0 {
               self.print_entry(entry);
               e_counter += 1;
            } else {
               break;
            }
         }
      }
   }

   // Internal function for printing boot IDs list
   fn print_boot_list(self)
   {
      println!("'{}'", self.filename);
      println!("Contains following Boot IDs:");
      for id in self.boot_ids {
         println!("{}", id);
      }
   }

   // Internal function for formatting and sort the entries,
   // returns a map <bootid, vector of entries to print>
   fn format_entry_list(&self) -> Result<HashMap<String, Vec<FormattedLogEntry>>, failure::Error>
   {
      let mut entry_list: HashMap<String, Vec<FormattedLogEntry>> = HashMap::new();

      // Check if boot filter should be used when creating HashMap keys
      let boot_list_filter = self.parse_opt.boot_filter();
      if boot_list_filter.len() != 0 {
         // only get entries from specified boot(s)
         for id in boot_list_filter {
            entry_list.insert(id.clone(), Vec::new());
         }
      } else {
         // get all entries
         for id in &self.boot_ids {
            entry_list.insert(id.clone(), Vec::new());
         }
      }

      // Loop through all entries and format them according to filters
      for e in &self.log_entries {

         // tmp variable for string comparison against filters, maybe not beautiful..
         let empty_string = "".to_string();
         let na = "N/A".to_string();

         let e_boot_id = match e._BOOT_ID {
            Some(ref id) => id,
            None => &empty_string
         };
         // Ignore entry if unknow boot ID
         if e_boot_id.is_empty() {
            eprintln!("Unable to format log entry, ignoring");
            continue;
         } else {
            if !entry_list.contains_key(e_boot_id) {
               // wrong boot ID, ignoring
               continue;
            }
         }

         let mut e_unit = match e.UNIT {
            Some(ref u_name) => u_name,
            None => { match e._SYSTEMD_UNIT {
               Some(ref systemd_unit_name) => systemd_unit_name,
               None => match e.SYSLOG_IDENTIFIER {
                  Some(ref syslog_name) => syslog_name,
                  None => &empty_string,
               }
            }},
         };

         if e_unit.is_empty() {
            eprintln!("Unable to get syslog identifier (unit name) for log entry");
            e_unit = &na;
         }

         // Check if kernel flag is set, if set check if entry is a kernel print
         if self.parse_opt.kernel_flag() {
            if e_unit != &"kernel".to_string() {
               continue;
            }
         }

         // Check against unit filter, if specified
         let unit_filter = &self.parse_opt.unit_filter();
         if unit_filter.len() != 0 {
            if !unit_filter.contains(&e_unit) {
               // not in unit list
               continue;
            }
         }

         // Check loglevel
         let log_level_lim = self.parse_opt.log_level();
         let mut e_log_level = match e.PRIORITY {
            Some(ref p) => p,
            None => &empty_string,
         };

         if e_log_level.is_empty() {
            eprintln!("Unable to get log level for entry");
            e_log_level = &na;
         } else {
            let log_level_int = e_log_level.parse::<u32>()?;
            if log_level_int > log_level_lim {
               // Log level higher (less priority) then filter
               continue;
            }
         }

         // Get sequence number
         let e_log_cursor_string = match e.__CURSOR {
            Some(ref c) => c,
            None => &empty_string
         };

         let e_seq_nr: String;
         if e_log_cursor_string.is_empty() {
            eprintln!("Unable to cursor string for entry");
            e_seq_nr = na.clone();
         } else {
            e_seq_nr = self.get_entry_nr(&e_log_cursor_string)?;
         }

         // Get PID
         let e_pid_nr = match &e._PID {
            Some(p) => p,
            None => &na
         };

         let b_o: &str = "(";
         let b_c: &str = ")";
         let unit_with_pid = e_unit.to_owned() + &b_o.to_owned() + &e_pid_nr.to_owned() + &b_c.to_owned();

         // Get timestamp
         let e_rt_ts = match e.__REALTIME_TIMESTAMP {
            Some(ref timestamp) => timestamp,
            None => &na
         };

         let e_rt_ts = e_rt_ts.parse::<i64>()?;

         // prepare for date filter (as timespan start/stop input)
         // add date filter here, now we have an int value to compare with

         let since_utc_s = e_rt_ts / 1000000;
         let formatted_timestamp = UTC.timestamp(since_utc_s, 0);
         let formatted_timestamp = formatted_timestamp.format("%Y-%m-%d %H:%M:%S").to_string();

         // Get entry log message
         let msg = match e.MESSAGE {
            Some(ref m) => m,
            None => &na
         };

         // Add handling of errno? For the future...

         // Add formatted entry to vector for correct boot
         match entry_list.get_mut(e_boot_id) {
            Some(boot) => boot.push(FormattedLogEntry{
               sequence_number: e_seq_nr,
               timestamp: formatted_timestamp,
               loglevel: e_log_level.clone(),
               unit_name: unit_with_pid,
               message: msg.clone()
               }),
            None => (),
         }


      } // for loop

      return Ok(entry_list);
   }

   // Print format header
   fn print_column_header(&self)
   {
      println!(
         "{0: <5}  {1: <20}  {2: <5}  {3: <18}  {4: }",
         "Seq#", "Datetime", "LVL", "Unit", "Message" );
   }

   // Print Current boot
   fn print_boot(&self, boot: &String, num: usize)
   {
      println!("- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -");
      println!("Boot : ID: {}, Number of parsed entries: {}", boot, num);
      println!("- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -");
   }

    // Function for the print of entry
   fn print_entry(&self, log_entry: &FormattedLogEntry) {
      println!("{0: <5}  {1: <20}  {2: <5}  {3: <18}   {4: }",
         log_entry.sequence_number, log_entry.timestamp,
         log_entry.loglevel, log_entry.unit_name, log_entry.message);
   }

   // function for returning the entry number
   fn get_entry_nr(&self, cursor_string: &String) -> Result<String, std::num::ParseIntError>
   {
      let cursor_v: Vec<&str> = cursor_string.split(";").collect();
      let pos = cursor_v[1].to_string();
      let seq_nr = pos.trim_start_matches("i=");

      match u32::from_str_radix(seq_nr, 16) {
         Ok(num) =>  return Ok(num.to_string()),
         Err(e) =>  return Err(e)
      };
   }
}
