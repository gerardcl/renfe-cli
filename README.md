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
7. Parses the HTML data, optionally sorts the connections and prints the timetable

```bash
$ renfe-cli -h
Usage: renfe-cli [options]

Options:
    -f ORIGIN           Set From origin station
    -t DESTINATION      Set To destination station
    -d, --day DAY       Set Day to search timetable for (default: today)
    -m, --month MONTH   Set Month to search timetable for (default: today's month)
    -y, --year YEAR     Set Year to search timetable for (default: today's year)
    -w, --wait SECONDS  Set Wait time in seconds for Renfe search result page (default: 2)
    -s, --sorted        Option to sort the timetable by Duration
    -h, --help          Print this help menu
```

### **Getting the timetable**

Let's show an example of minimal inputs (origin and destination stations) with specific date:

```bash
$ renfe-cli -f Barc -t Mad -d 27
Loading stations from Renfe web
Provided input 'Barc' station matches with 'Barcelona (ALL) '...continue
Provided input 'Mad' station matches with 'Madrid (ALL) '...continue
Today is: 2023-11-26
Searching timetable for date: 2023-11-27
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
AVE          |    05.50     |    09.10     | 3 h. 20 min.
-----------------------------------------------------------
AVE          |    06.20     |    08.50     | 2 h. 30 min.
-----------------------------------------------------------
AVLO         |    06.35     |    09.20     | 2 h. 45 min.
-----------------------------------------------------------
AVE          |    07.00     |    09.30     | 2 h. 30 min.
-----------------------------------------------------------
AVE          |    07.40     |    10.10     | 2 h. 30 min.
-----------------------------------------------------------
LD-AVE       |    07.45     |    15.35     | 7 h. 50 min.
-----------------------------------------------------------
AVE          |    08.00     |    11.12     | 3 h. 12 min.
-----------------------------------------------------------
AVE          |    08.25     |    10.55     | 2 h. 30 min.
-----------------------------------------------------------
REG.EXP.     |    08.43     |    18.09     | 9 h. 26 min.
-----------------------------------------------------------
AVE          |    09.00     |    11.45     | 2 h. 45 min.
-----------------------------------------------------------
AVLO         |    10.00     |    13.17     | 3 h. 17 min.
-----------------------------------------------------------
AVE          |    11.00     |    13.45     | 2 h. 45 min.
-----------------------------------------------------------
AVE          |    12.00     |    15.12     | 3 h. 12 min.
-----------------------------------------------------------
AVE INT      |    12.50     |    15.45     | 2 h. 55 min.
-----------------------------------------------------------
AVE          |    13.25     |    15.54     | 2 h. 29 min.
-----------------------------------------------------------
AVE          |    14.00     |    17.12     | 3 h. 12 min.
-----------------------------------------------------------
AVLO         |    15.00     |    17.45     | 2 h. 45 min.
-----------------------------------------------------------
AVE          |    15.25     |    17.55     | 2 h. 30 min.
-----------------------------------------------------------
AVE          |    16.00     |    19.12     | 3 h. 12 min.
-----------------------------------------------------------
AVE          |    16.25     |    18.55     | 2 h. 30 min.
-----------------------------------------------------------
AVE          |    17.00     |    19.45     | 2 h. 45 min.
-----------------------------------------------------------
AVE          |    17.25     |    19.55     | 2 h. 30 min.
-----------------------------------------------------------
AVE          |    18.00     |    21.12     | 3 h. 12 min.
-----------------------------------------------------------
AVE          |    18.25     |    20.55     | 2 h. 30 min.
-----------------------------------------------------------
AVE          |    18.40     |    21.45     | 3 h. 5 min.
-----------------------------------------------------------
AVE          |    19.25     |    21.55     | 2 h. 30 min.
-----------------------------------------------------------
AVE          |    20.00     |    23.12     | 3 h. 12 min.
-----------------------------------------------------------
AVLO         |    21.00     |    23.45     | 2 h. 45 min.
-----------------------------------------------------------
AVE          |    21.25     |    23.55     | 2 h. 30 min.
===========================================================
```

## Usage (Library)

`renfe-cli` can be imported as a python package into your project, offering utilities when willing to deal with the Renfe search web site.

```bash
$ python
Python 3.8.18 (default, Aug 25 2023, 13:20:30)
[GCC 11.4.0] on linux
Type "help", "copyright", "credits" or "license" for more information.
>>> import renfe_cli
>>> renfe = renfe_cli.
renfe_cli.Renfe(             renfe_cli.main(              renfe_cli.print_timetable(   renfe_cli.renfe_cli          renfe_cli.search_timetable(
>>> renfe = renfe_cli.Renfe()
Loading stations from Renfe web
>>> renfe.
renfe.filter_station(  renfe.stations_match(
>>> renfe.stations_match("Bar")
['Barcelona (ALL) ', 'Padrón-Barbanza']
```

---

## Contribute or Report with Issues

If Renfe's website is changed or you find any issue to be fixed or nice enhancements to have, please: [create an issue](https://github.com/gerardcl/renfe-cli/issues).

### Development

This project makes use of Rust bindings for the Python interpreter thanks to [pyo3](https://pyo3.rs). It is already available as a dependency.

To develop, build and publish, this project makes use of [maturin](https://www.maturin.rs/) project. See [usage](https://www.maturin.rs/#usage).

Example of first time working with this repository:

```bash
$ git clone https://github.com/gerardcl/renfe-cli.git && cd renfe-cli
$ python -m venv venv
$ . venv/bin/activate
$ pip install -U pip
$ pip install -U maturin
$ maturing develop
🔗 Found pyo3 bindings with abi3 support for Python ≥ 3.7
🐍 Not using a specific python interpreter
📡 Using build options features from pyproject.toml
   Compiling renfe-cli v4.1.0 (/path/to/renfe-cli)
    Finished dev [unoptimized + debuginfo] target(s) in 7.07s
📦 Built wheel for abi3 Python ≥ 3.7 to /tmp/.tmpDsjowL/renfe_cli-4.1.0-cp37-abi3-linux_x86_64.whl
🛠 Installed renfe-cli-4.1.0
```

Maturin takes care of compiling the rust code, generating the bindings for python and installing the package for local use (as library or binary/CLI).
