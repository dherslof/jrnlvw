# jrnlvw - journal viewer
Simple parser for viewing systemd journal files (in json format). Mostly done as Rust practice but the tool can actually be useful, hopefully for someone more then myself! 

## Usage
Following examples expects `jrnlvw` to be installed.  

View a logfile: 
```bash 
$ jrnlvw <loggile>
```
List boots in logfile: 
```bash
$ jrnlvw <logfile> --list-boots
```

Print the 25 first kernel log entries from all boots in file:
```bash
$ jrnlvw <logfile> -n 25 --kernel
```

More help and filters: 
```bash 
$ jrnlvw --help
```
### Filters
Current implemented filters are: 
* kernel -- Only display kernel logs
* list-boots -- Display a list of boot IDs in logfile
* boot -- Display logs from specified boot ID
* number -- Display max amount of logs from every boot(s)
* priority -- Only display logs with a loglevel higher then specified
* unit -- Only display logs from specified unit(s)

Different filters can be used for combined filtering.
