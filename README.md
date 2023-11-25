# RENFE TIMETABLES CLI

Get faster RENFE Spanish trains timetables in your terminal, with Python3.7+ support.
No longer need to open the browser! Just keep using your terminal 😀

**HIGHLIGHT**: Although it is provided as a python package (script and library), it is entirely written in Rust (since v4.0.0).

**NOTE** since I am more often using Rodalies trains I have created [rodalies-cli](https://github.com/gerardcl/rodalies-cli). I hope you like it too!

See the [changelog](https://github.com/gerardcl/renfe-cli/blob/master/CHANGELOG.md).

## Installation

Install Python CLI package [renfe-cli](https://pypi.org/project/renfe-cli/)

```bash
$ pip install renfe-cli --upgrade
```

## Usage (CLI)

This CLI behaves as a person/bot going through the official renfe.com site, using headless chrome browsers.
If the headless chrome browser is not found it will download it.

The navigation through the site happens in the following steps:

1. Writes down and selects origin station
2. Writes down and selects destination station
3. Writes down and selects the day to search for
4. Writes down and selects the month to search for
5. Writes down and selects the year to search for
6. Clicks on search button
7. Parses the HTML data and prints the timetable

```bash
$ renfe-cli -h
Usage: renfe-cli [options]

Options:
    -f ORIGIN           Set From origin station
    -t DESTINATION      Set To destination station
    -d, --day DAY       Set Day to search timetable for
    -m, --month MONTH   Set Month to search timetable for
    -y, --year YEAR     Set Year to search timetable for
    -h, --help          print this help menu
```

### **Getting the timetable**

Timetable defaults to `today`, from `Sils` (ID is `79202`) to `Barcelona Passeig de Gràcia` (ID is `71802`):

```bash
$ renfe-cli
Today is: 2021-05-22
Searching timetable for date: 2021-5-22
From SILS to BARCELONA-PASSEIG DE GRACIA
Be patient, navigating through renfe site now...
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

---

## Usage (Library)

TBD

## Contribute or Report with Issues

If Renfe's website is changed or you find any issue to be fixed or nice enhancements to have, please: [create an issue](https://github.com/gerardcl/renfe-cli/issues).
