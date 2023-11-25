[![CICD](https://github.com/gerardcl/renfe-cli/actions/workflows/CICD.yml/badge.svg)](https://github.com/gerardcl/renfe-cli/actions/workflows/CICD.yml)

# Renfe Timetables CLI

Get faster Renfe trains timetables in your terminal, with Python3.7+ support.
No longer need to open the browser! Just keep using your terminal 😀

`renfe-cli` is written in [Rust](https://www.rust-lang.org/) (since v4.0.0) and published to [pypi.org](https://pypi.org/project/renfe-cli/) as a Python package (CLI and library).

See the [changelog](https://github.com/gerardcl/renfe-cli/blob/master/CHANGELOG.md).

**NOTE** since I am more often using Rodalies trains I have created [rodalies-cli](https://github.com/gerardcl/rodalies-cli). I hope you like it too!

## Installation

Install Python CLI package [renfe-cli](https://pypi.org/project/renfe-cli/)

```bash
pip install renfe-cli --upgrade
```

## Usage (CLI)

This CLI behaves as a person/bot going through the official renfe.com search site, using headless chrome browser.
If the headless chrome browser is not found it will be downloaded.

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

In this new major release there is still no interactive mode nor defaults; one must provide all inputs, like:

```bash
$ renfe-cli -f Tarr -t Mad -d 27 -m 11 -y 2023
loading headless chrome browser
navigating to renfe timetable search page
waiting for search page
adding origin station
adding destination station
adding day
adding month
adding year
searching timetable
got timetable page
loading timetable
=========================TIMETABLE=========================
Train        |   Departure  |   Arrival    | Duration
-----------------------------------------------------------
AVE          |    06.25     |    09.10     | 2 h. 45 min.
-----------------------------------------------------------
LD-AVE       |    08.22     |    15.35     | 7 h. 13 min.
-----------------------------------------------------------
AVE          |    08.34     |    11.12     | 2 h. 38 min.
-----------------------------------------------------------
REG.EXP.     |    10.11     |    18.09     | 7 h. 58 min.
-----------------------------------------------------------
AVLO         |    10.34     |    13.17     | 2 h. 43 min.
-----------------------------------------------------------
LD-AVE       |    10.51     |    15.35     | 4 h. 44 min.
-----------------------------------------------------------
AVE          |    12.34     |    15.12     | 2 h. 38 min.
-----------------------------------------------------------
AVE INT      |    13.22     |    15.45     | 2 h. 23 min.
-----------------------------------------------------------
AVE          |    14.34     |    17.12     | 2 h. 38 min.
-----------------------------------------------------------
AVE          |    16.34     |    19.12     | 2 h. 38 min.
-----------------------------------------------------------
AVE          |    18.34     |    21.12     | 2 h. 38 min.
-----------------------------------------------------------
AVE          |    19.14     |    21.45     | 2 h. 31 min.
-----------------------------------------------------------
AVE          |    20.34     |    23.12     | 2 h. 38 min.
===========================================================
```

---

## Usage (Library)

TBD

## Contribute or Report with Issues

If Renfe's website is changed or you find any issue to be fixed or nice enhancements to have, please: [create an issue](https://github.com/gerardcl/renfe-cli/issues).
