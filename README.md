[![CICD](https://github.com/gerardcl/renfe-cli/actions/workflows/CICD.yml/badge.svg)](https://github.com/gerardcl/renfe-cli/actions/workflows/CICD.yml)

# Renfe Timetables CLI

Get faster Renfe trains timetables in your terminal, with Python3.7+ support.
No longer need to open the browser! Just keep using your terminal 😀

`renfe-cli` is written in [Rust](https://www.rust-lang.org/) (since v4.0.0) and published to [pypi.org](https://pypi.org/project/renfe-cli/) as a Python package (CLI and library).

See the [changelog](https://github.com/gerardcl/renfe-cli/blob/master/CHANGELOG.md).

**NOTE** since I am more often using Rodalies trains I have created [rodalies-cli](https://github.com/gerardcl/rodalies-cli). I hope you like it too!

  **DISCLAIMER**: Renfe's GTFS dataset might not be in sync with autonomic train schedules (e.g. Rodalies de la Generalitat de Catalunya), hence Renfe Cercanias train types (e.g.: REGIONAL or MD type) might not be accurate. For that, please use autonomic data/apps (.e.g: [rodalies-cli](https://github.com/gerardcl/rodalies-cli)).

## Installation

Install Python CLI package [renfe-cli](https://pypi.org/project/renfe-cli/)

```bash
pip install renfe-cli --upgrade
```

## Usage (CLI)

The CLI uses the official and latest Renfe's GTFS dataset, from [Horarios de alta velocidad, larga distancia y media distancia](https://data.renfe.com/dataset/horarios-de-alta-velocidad-larga-distancia-y-media-distancia).

```bash
$ renfe-cli -h
Usage: renfe-cli [options]

Options:
    -f ORIGIN           Set From origin station
    -t DESTINATION      Set To destination station
    -d, --day DAY       Set the Day (default: today's day)
    -m, --month MONTH   Set the Month (default: today's month)
    -y, --year YEAR     Set the Year (default: today's year)
    -s, --sort          Option to sort the timetable by Duration
    -h, --help          Print this help menu
```

### **Getting the timetable**

Let's show an example of minimal inputs (origin and destination stations) with specific date:

```bash
$ renfe-cli  -f girona -t "puerta de atocha" -d 30
Loading GTFS data from Renfe web
Provided input 'girona' does a match with 'Estación de tren Girona'
Provided input 'puerta de atocha' does a match with 'Estación de tren Madrid-Puerta de Atocha'
Today is: 2024-9-29
Searching timetable for date: 2024-9-30
Origin station: Estación de tren Girona
Destination station: Estación de tren Madrid-Puerta de Atocha

=========================TIMETABLE=========================
  Train        |   Departure  |   Arrival    |   Duration
-----------------------------------------------------------
   AVLO        |    05:46     |    09:20     |    03:34
-----------------------------------------------------------
   AVE         |    06:41     |    10:10     |    03:29
-----------------------------------------------------------
   AVE         |    08:11     |    11:45     |    03:34
-----------------------------------------------------------
   AVE INT     |    11:59     |    15:45     |    03:46
-----------------------------------------------------------
   AVE         |    15:11     |    19:12     |    04:01
-----------------------------------------------------------
   AVE         |    17:51     |    21:45     |    03:54
===========================================================
```

## Usage (Library)

`renfe-cli` can be imported as a python package into your project, offering utilities when willing to deal with the Renfe search web site.

```bash
$ python
Python 3.12.6 (main, Sep  8 2024, 13:18:56) [GCC 14.2.1 20240805] on linux
Type "help", "copyright", "credits" or "license" for more information.
>>> import renfe_cli
>>> renfe = renfe_cli.
renfe_cli.Renfe()    renfe_cli.Schedule(  renfe_cli.Station(   renfe_cli.main()     renfe_cli.renfe_cli  
>>> renfe = renfe_cli.Renfe()
Loading GTFS data from Renfe web
>>> renfe.filter_station("madrid")
Traceback (most recent call last):
  File "<stdin>", line 1, in <module>
ValueError: Provided input 'madrid' does match with '[Station { name: "Estación de tren Madrid-Puerta de Atocha", id: "60000" }, Station { name: "Estación de tren Madrid - Atocha Cercanias", id: "18000" }, Station { name: "Estación de tren Madrid-Principe Pio", id: "10000" }, Station { name: "Estación de tren Madrid-Ramon Y Cajal", id: "97201" }, Station { name: "Estación de tren Madrid-Nuevos Ministerios", id: "18002" }, Station { name: "Estación de tren Madrid-Chamartin", id: "17000" }, Station { name: "Estación de tren Madrid-Recoletos", id: "18001" }]' -> There must be ONLY one match
>>> renfe.filter_station("girona")
Provided input 'girona' does a match with 'Station { name: "Estación de tren Girona", id: "79300" }'
<builtins.Station object at 0x77f04173d070>
>>> renfe.print_timetable()

No schedules available...won't print timetable.
>>> renfe.set_train_schedules("79300", "60000", 30, 9, 2024, False)
>>> renfe.print_timetable()

=========================TIMETABLE=========================
  Train        |   Departure  |   Arrival    |   Duration
-----------------------------------------------------------
   AVLO        |    05:46     |    09:20     |    03:34
-----------------------------------------------------------
   AVE         |    06:41     |    10:10     |    03:29
-----------------------------------------------------------
   AVE         |    08:11     |    11:45     |    03:34
-----------------------------------------------------------
   AVE INT     |    11:59     |    15:45     |    03:46
-----------------------------------------------------------
   AVE         |    15:11     |    19:12     |    04:01
-----------------------------------------------------------
   AVE         |    17:51     |    21:45     |    03:54
===========================================================
>>> ...
```

---

## Contribute or Report with Issues

If Renfe's GTFS dataset is being kept not up to date or you find any issue to be fixed or nice enhancements to have, please: [create an issue](https://github.com/gerardcl/renfe-cli/issues).

### Development

This project makes use of Rust bindings for the Python interpreter thanks to [pyo3](https://pyo3.rs). It is already available as a dependency.

To develop, build and publish, this project makes use of [maturin](https://www.maturin.rs/) project. See [usage](https://www.maturin.rs/#usage).

Example of first time working with this repository:

```bash
$ git clone https://github.com/gerardcl/renfe-cli.git && cd renfe-cli
$ python -m venv .venv
$ . .venv/bin/activate
$ pip install -U pip
$ pip install -U maturin
$ maturin develop
🔗 Found pyo3 bindings with abi3 support for Python ≥ 3.7
🐍 Not using a specific python interpreter
📡 Using build options features from pyproject.toml
   Compiling renfe-cli v4.1.0 (/path/to/renfe-cli)
    Finished dev [unoptimized + debuginfo] target(s) in 7.07s
📦 Built wheel for abi3 Python ≥ 3.7 to /tmp/.tmpDsjowL/renfe_cli-4.1.0-cp37-abi3-linux_x86_64.whl
🛠 Installed renfe-cli-4.1.0
```

Maturin takes care of compiling the rust code, generating the bindings for python and installing the package for local use (as library or binary/CLI).
