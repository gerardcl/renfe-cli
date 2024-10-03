[![CICD](https://github.com/gerardcl/renfe-cli/actions/workflows/CICD.yml/badge.svg)](https://github.com/gerardcl/renfe-cli/actions/workflows/CICD.yml)

# Renfe Timetables CLI

Get faster Renfe trains timetables in your terminal, with Python3.8+ support.
No longer need to open the browser! Just keep using your terminal 😀

It supports both [Horarios de alta velocidad, larga distancia y media distancia](https://data.renfe.com/dataset/horarios-de-alta-velocidad-larga-distancia-y-media-distancia) (default option, as in the web) and [Renfe Cercanías](https://data.renfe.com/dataset/horarios-cercanias)  GTFS datasets.

`renfe-cli` is written in [Rust](https://www.rust-lang.org/) (since v4.0.0) and published to [pypi.org](https://pypi.org/project/renfe-cli/) as a Python package (CLI and library).

It is provided as a Python package due to historical reasons, but was ported to Rust to showcase Rust's interoperability and performance improvements that can offer to the Python ecosystem. Nevertheless, one can optionally use the built [renfe-cli](https://crates.io/crates/renfe-cli) crate that is publised to crates.io.

See the [changelog](https://github.com/gerardcl/renfe-cli/blob/master/CHANGELOG.md).

**NOTE** since I am more often using Rodalies trains I have created [rodalies-cli](https://github.com/gerardcl/rodalies-cli). I hope you like it too!

  **DISCLAIMER**: Renfe's GTFS dataset might not be in sync with autonomic train schedules systems (e.g. Rodalies de la Generalitat de Catalunya), hence Renfe Cercanias train types (e.g.: REGIONAL or MD type) might not be accurate, or when using the `cercanias` flag you won't find timetables for the stations belonging to autonomic systems. For that, please use autonomic data/apps (.e.g: [rodalies-cli](https://github.com/gerardcl/rodalies-cli)).

## Installation

### Python package

Install Python CLI package [renfe-cli](https://pypi.org/project/renfe-cli/)

```bash
pip install renfe-cli --upgrade
```

### Rust crate (optional)

Install the Rust crate [renfe-cli](https://crates.io/crates/renfe-cli)

```bash
cargo install renfe-cli
```

## Usage (CLI)

The CLI uses the official and latest Renfe's GTFS dataset, from [Horarios de alta velocidad, larga distancia y media distancia](https://data.renfe.com/dataset/horarios-de-alta-velocidad-larga-distancia-y-media-distancia), by default. Optionally, one can enable searching over [Renfe Cercanías GTFS dataset](https://data.renfe.com/dataset/horarios-cercanias) (expect longer load time in this case).

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
    -c, --cercanias     Option to search over Renfe Cercanías
    -h, --help          Print this help menu
```

### **Getting the timetable**

Let's show an example of minimal inputs (origin and destination stations) with specific date and default GTFS dataset:

```bash
$ renfe-cli  -f girona -t "puerta de atocha" -d 30
Loading default GTFS data from Renfe web - Alta velocidad, Larga distancia y Media distancia
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

Let's show an example using Renfe Cercanías GTFS dataset:

```bash
$ renfe-cli -f chamartín -t "tres cantos" -c
Loading Cercanías GTFS data from Renfe web - long load time
Provided input 'chamartín' does a match with 'Station { name: "Estación de tren Madrid-Chamartín-Clara Campoamor", id: "17000" }'
Provided input 'tres cantos' does a match with 'Station { name: "Estación de tren Tres Cantos (apt)", id: "17004" }'
Today is: 2024-10-2
Searching timetable for date: 2024-10-2
Origin station: Estación de tren Madrid-Chamartín-Clara Campoamor
Destination station: Estación de tren Tres Cantos (apt)

=========================TIMETABLE=========================
  Train        |   Departure  |   Arrival    |   Duration    
-----------------------------------------------------------
   C4b         |    05:06     |    05:22     |    00:16     
-----------------------------------------------------------
   C4b         |    05:38     |    05:55     |    00:17     
-----------------------------------------------------------
   C4b         |    06:10     |    06:27     |    00:17     
-----------------------------------------------------------
.........
.........
-----------------------------------------------------------
   C4b         |    21:56     |    22:13     |    00:17     
-----------------------------------------------------------
   C4b         |    22:20     |    22:37     |    00:17     
-----------------------------------------------------------
   C4b         |    23:16     |    23:33     |    00:17     
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
Traceback (most recent call last):
  File "<stdin>", line 1, in <module>
TypeError: Renfe.__new__() missing 1 required positional argument: 'cercanias'
>>> renfe = renfe_cli.Renfe(False)
Loading default GTFS data from Renfe web - Alta velocidad, Larga distancia y Media distancia
GTFS data:
  Read in 2171 ms
  Stops: 793
  Routes: 644
  Trips: 4150
  Agencies: 1
  Shapes: 0
  Fare attributes: 0
  Feed info: 0
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
   Compiling renfe-cli v5.1.0 (/path/to/renfe-cli)
    Finished dev [unoptimized + debuginfo] target(s) in 7.07s
📦 Built wheel for abi3 Python ≥ 3.7 to /tmp/.tmpDsjowL/renfe_cli-5.1.0-cp37-abi3-linux_x86_64.whl
🛠 Installed renfe-cli-5.1.0
```

Maturin takes care of compiling the rust code, generating the bindings for python and installing the package for local use (as library or binary/CLI).
