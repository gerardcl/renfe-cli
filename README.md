[![CICD](https://github.com/gerardcl/renfe-cli/actions/workflows/CICD.yml/badge.svg)](https://github.com/gerardcl/renfe-cli/actions/workflows/CICD.yml)

# Renfe Timetables CLI

Get faster Renfe trains timetables in your terminal, with Python 3.10+ support.
No longer need to open the browser! Just keep using your terminal 😀

It supports both [Horarios de alta velocidad, larga distancia y media distancia](https://data.renfe.com/dataset/horarios-de-alta-velocidad-larga-distancia-y-media-distancia) (default option, as in the web) and [Renfe Cercanías](https://data.renfe.com/dataset/horarios-cercanias) GTFS datasets. Timetable searches include connections with up to three transfers. The router favors shorter journeys and applies a transfer penalty so a marginal time saving does not produce an unnecessarily complicated itinerary.

`renfe-cli` is written in [Rust](https://www.rust-lang.org/) (since v4.0.0) and published to [pypi.org](https://pypi.org/project/renfe-cli/) as a Python package (CLI and library).

It is provided as a Python package due to historical reasons, but was ported to Rust to showcase Rust's interoperability and performance improvements that can offer to the Python ecosystem. Nevertheless, one can optionally use the built [renfe-cli](https://crates.io/crates/renfe-cli) crate that is publised to crates.io.

See the [changelog](https://github.com/gerardcl/renfe-cli/blob/master/CHANGELOG.md).

**NOTE** since I am more often using Rodalies trains I have created [rodalies-cli](https://github.com/gerardcl/rodalies-cli). I hope you like it too!

  **DISCLAIMER**: Renfe's GTFS dataset might not be in sync with autonomic train schedules systems (e.g. Rodalies de la Generalitat de Catalunya), hence Renfe Cercanias train types (e.g.: REGIONAL or MD type) might not be accurate, or when using the `cercanias` flag you won't find timetables for the stations belonging to autonomic systems. For that, please use autonomic data/apps (.e.g: [rodalies-cli](https://github.com/gerardcl/rodalies-cli)).

## Installation

### Python package

Install Python CLI package [renfe-cli](https://pypi.org/project/renfe-cli/)

```bash
uv tool install renfe-cli --upgrade
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
    -V, --version       Print version information
```

At startup, `renfe-cli` prints its running version and checks for a newer
release. Python installations check PyPI, while the native Rust binary checks
crates.io. Use `renfe-cli --version` to print only the running version.

### **Getting the timetable**

Let's show an example of minimal inputs (origin and destination stations) with specific date and default GTFS dataset:

```bash
$ renfe-cli  -f girona -t "puerta de atocha" -d 30
Loading default GTFS data from Renfe web - Alta velocidad, Larga distancia y Media distancia
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

### Selecting a station

Station names are matched case-insensitively and may be abbreviated. If the
provided text matches more than one station, the CLI sorts the matches by name
and asks which station to use. Select it by entering its 1-based ordinal number:

```text
$ renfe-cli -f madrid -t girona
Multiple stations match 'madrid':
  1. Estación de tren Madrid - Atocha Cercanias
  2. Estación de tren Madrid-Chamartin
  3. Estación de tren Madrid-Nuevos Ministerios
  4. Estación de tren Madrid-Principe Pio
  5. Estación de tren Madrid-Puerta de Atocha
  6. Estación de tren Madrid-Ramon Y Cajal
  7. Estación de tren Madrid-Recoletos
Select a station [1-7]: 5
```

Invalid numbers are rejected and the CLI asks again. If the provided text does
not match any station, it reports the input without presenting a selection:

```text
ValueError: Provided input 'unknown' does not match any station name
```

Journeys with connections include the details of every transfer directly
below the timetable row. The arrival and departure are the times at which the
transfer starts and ends; the duration is the total time available to change
trains. Transfers between different stops show both station names:

```text
      Transfer 1 at Madrid-Chamartín: arrive 10:05, depart 10:20 (00:15)
      Transfer 2 at Madrid-Atocha → Madrid-Puerta de Atocha: arrive 10:45, depart 11:10 (00:25)
```

Downloaded feeds are converted to routing data and cached in the operating
system's user cache directory. On later runs, `renfe-cli` sends Renfe the
stored HTTP `ETag`; when the feed has not changed, Renfe returns `304 Not
Modified` and the processed data is loaded directly from the local cache.
Set `RENFE_CLI_CACHE_DIR` to use a custom cache location.

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
>>> len(renfe.stations_match("madrid"))
7
>>> len(renfe.stations_match("girona"))
1
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

This project uses Rust bindings for the Python interpreter through [PyO3](https://pyo3.rs) and [Maturin](https://www.maturin.rs/) to build and install the Python package.

This project uses [Just](https://github.com/casey/just) as the command runner for local development.

### Prerequisites

Make sure the following stack is installed:

    [Rust](https://www.rust-lang.org/)
    [uv](https://docs.astral.sh/uv/getting-started/installation/)
    [Just](https://github.com/casey/just)
    [prek](https://prek.j178.dev/)

### Setup and workflows

```bash
git clone https://github.com/gerardcl/renfe-cli.git
cd renfe-cli
just dev-setup
just # to see available commands
```
