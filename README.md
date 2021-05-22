[![Quality Gate Status](https://sonarcloud.io/api/project_badges/measure?project=gerardcl_renfe-cli&metric=alert_status)](https://sonarcloud.io/dashboard?id=gerardcl_renfe-cli) [![Build Python Package](https://github.com/gerardcl/renfe-cli/actions/workflows/python-package.yml/badge.svg)](https://github.com/gerardcl/renfe-cli/actions/workflows/python-package.yml)

# RENFE TIMETABLES CLI

Get faster RENFE Spanish trains timetables in your terminal, with Python3.8+ (see [builds](https://travis-ci.org/gerardcl/renfe-cli)).
No longer need to open the browser! Just keep using your terminal :)

See the [changelog](https://github.com/gerardcl/renfe-cli/blob/master/CHANGELOG.md).

## Installation

Install Python CLI package [renfe-cli](https://pypi.org/project/renfe-cli/)

```bash
$ pip install renfe-cli --upgrade
```

## Usage

This CLI behaves as a person going thourgh the official renfe.com site, selecting origin and destianion stations.
This also considers by default searching for today, or to which day to search the timetable from today's day as reference (like a click on next day).

```bash
$ renfe-cli -h
Usage: renfe-cli [options]

Options:
  -h, --help            show this help message and exit
  -d DAYS, --days=DAYS  number of days from today to get the timetable
  -o ORIGIN, --origin=ORIGIN
                        from/origin ID of the train station. Use flag '-s
                        <possible station name>' in order to search for IDs
  -t TO, --to=TO        to/destination ID of the train station. Use flag '-s
                        <possible station name>' in order to search for IDs
  -s SEARCH, --search=SEARCH
                        you need to get the stations IDs, searching by names;
                        in order to apply right inputs for origins and/or
                        destinations
  -l LOGGING_LEVEL, --logging-level=LOGGING_LEVEL
                        logging level defaults to warning and possible values
                        are: debug, info, warning, error and critical
  -f LOGGING_FILE, --logging-file=LOGGING_FILE
                        logging file name is required if you want to submit an
                        issue with more information
  -u, --update-config   change your origin and destination stations to
                        defaults when loading this flag
```

### **Searching for IDs of train stations**

 When using search functionality, it will provide you with the IDs (to use as an origin or destiation train station) of the stations that are similar to the input text to search. Example:

```bash
$ renfe-cli -s sil
Today is: 2021-05-22
Searching stations like: sil
SILLA: 64200
SILS: 79202
SAN ESTEVO DO SIL: 22003
SAN PEDRO DO SIL: 22004
LE PEAGE DE ROUSSILLON: 00245
SILLE LE GUILLAUME: 00818
```

### **Getting the timetable**

Timetable defaults to `today`, from `Sils` (ID is `79202`) to `Barcelona Passeig de Gràcia` (ID is `71802`):

```bash
$ renfe-cli
Today is: 2021-05-22
Searching timetable for date: 2021-5-22
From SILS to BARCELONA-PASSEIG DE GRACIA
...this might take some seconds depending on the Renfe site speed...
=======================TIMETABLE======================
 Train      | Departure  | Arrival    | Duration
------------------------------------------------------
 MD         | 06.45      | 07.49      | 1 h. 4 min.
------------------------------------------------------
 MD         | 07.30      | 08.34      | 1 h. 4 min.
------------------------------------------------------
 REGIONAL   | 08.29      | 09.34      | 1 h. 5 min.
------------------------------------------------------
 MD         | 09.05      | 10.04      | 59 min.
------------------------------------------------------
 REGIONAL   | 09.59      | 11.04      | 1 h. 5 min.
------------------------------------------------------
 MD         | 10.35      | 11.34      | 59 min.
------------------------------------------------------
 REGIONAL   | 11.59      | 13.04      | 1 h. 5 min.
------------------------------------------------------
 MD         | 12.35      | 13.34      | 59 min.
------------------------------------------------------
 REGIONAL   | 13.59      | 15.04      | 1 h. 5 min.
------------------------------------------------------
 MD         | 14.35      | 15.34      | 59 min.
------------------------------------------------------
 REGIONAL   | 15.59      | 17.04      | 1 h. 5 min.
------------------------------------------------------
 MD         | 16.35      | 17.34      | 59 min.
------------------------------------------------------
 REGIONAL   | 17.59      | 19.04      | 1 h. 5 min.
------------------------------------------------------
 MD         | 18.35      | 19.34      | 59 min.
------------------------------------------------------
 MD         | 19.35      | 20.34      | 59 min.
------------------------------------------------------
 REGIONAL   | 21.09      | 22.14      | 1 h. 5 min.
------------------------------------------------------
 REGIONAL   | 21.42      | 22.46      | 1 h. 4 min.
======================================================
```

Which would be the same as:

```bash
$ renfe-cli -d 0 -o 79202 -t BARCE
```

### **Changing default origin and/or destination stations**
In order to change default timetable stations you just need to add `-u` flag in the CLI, and next time you won't need to add the used `-o` and `-t` params:

```bash
$ renfe-cli -o MADRI -t BARCE -u
```

If changing defaults, a file is created under user's home directory in file `~/.renfe_default_stations.json`

---

## Issues

If Renfe's website is changed or you find any issue or enhancements, please: [create an issue](https://github.com/gerardcl/renfe-cli/issues)

## Installation alternatives (getting latest source code)

If you want to install latest source code:

```bash
$ pip install git+http://github.com/gerardcl/renfe-cli
```

or

```bash
$ git clone git://github.com/gerardcl/renfe-cli
$ cd renfe-cli
$ python setup.py install
```
