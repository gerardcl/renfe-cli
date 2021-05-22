[![Quality Gate Status](https://sonarcloud.io/api/project_badges/measure?project=gerardcl_renfe-cli&metric=alert_status)](https://sonarcloud.io/dashboard?id=gerardcl_renfe-cli)

RENFE TIMETABLES CLI
====================

Get faster RENFE Spanish trains timetables in your terminal. Works on Linux, OSX and Windows Python3 (see [builds](https://travis-ci.org/gerardcl/renfe-cli)).

See the [changelog](https://github.com/gerardcl/renfe-cli/blob/master/CHANGELOG.md).


Installation
------------

Install Python CLI package [renfe-cli](https://pypi.org/project/renfe-cli/)

```
$ pip install renfe-cli --upgrade
```

Usage
-----
```
$ renfe-cli -h
Usage: renfe-cli [options]

Options:
  -h, --help            show this help message and exit
  -y YEAR, --year=YEAR  year selected to get the timetable from
  -m MONTH, --month=MONTH
                        month of the year to get the timetable from
  -d DAY, --day=DAY     day of the month to get the timetable from
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

```
$ renfe-cli -s sil
Today is: 2019-05-05
Searching stations like: sil
SANESTEVODOSIL: 22003
SANPEDRODOSIL: 22004
SILLA: 64200
SILS: 79202

$ renfe-cli -s barc
Today is: 2019-05-05
Searching stations like: barc
BARCELONA: 97007
BARCELONA(TODAS): BARCE
BARCELONA-ARCDETRIOMF: 78804
BARCELONA-PLAÇADECATALUNYA: 78805
BARCELONA-TORREDELBARO: 78801
BARCELOS: 94024
BARCENA: 14206
CEUTA-BARCO: 99126
ELBARCENAL: 05644
OBARCODEVALDEORRAS: 20211
TANGERMED-BARCO: 99124
TANGERVILLE-BARCO: 99125
VILANOVADELABARCA: 75102
```

### **Getting the timetable**

Timetable defaults to `today`, from `Sils` (ID is `79202`) to `Barcelona` (ID is `BARCE`):

```
$ renfe-cli
Today is: 2019-05-05
Searching timetable for date: 2019-05-05
From 79202 to BARCE
=================== TIMETABLE ====================
   Tren / Recorrido  Salida  Llegada      Duración
0          15060 MD    7.23     8.39  1 h. 16 min.
1          15064 MD    8.27     9.39  1 h. 12 min.
2    15806 REGIONAL   10.20    11.39  1 h. 19 min.
3    15808 REGIONAL   11.43    13.09  1 h. 26 min.
4          15070 MD   12.27    13.39  1 h. 12 min.
5    15810 REGIONAL   13.33    14.52  1 h. 19 min.
6          15090 MD   14.27    15.39  1 h. 12 min.
7    15812 REGIONAL   15.39    17.09  1 h. 30 min.
8          15072 MD   16.57    18.09  1 h. 12 min.
9    15814 REGIONAL   17.39    18.55  1 h. 16 min.
10         15018 MD   18.06    19.18  1 h. 12 min.
11         15918 MD   18.57    20.09  1 h. 12 min.
12         15074 MD   19.45    20.55  1 h. 10 min.
13   15848 REGIONAL   20.01    21.09   1 h. 8 min.
14   15850 REGIONAL   20.59    22.19  1 h. 20 min.
15   15822 REGIONAL   21.38    22.55  1 h. 17 min.
==================================================
```

Which would be the same as:

```
$ renfe-cli -y 2019 -m 5 -d 5 -o 79202 -t BARCE
Today is: 2019-05-05
Searching timetable for date: 2019-05-05
From 79202 to BARCE
=================== TIMETABLE ====================
   Tren / Recorrido  Salida  Llegada      Duración
0          15060 MD    7.23     8.39  1 h. 16 min.
1          15064 MD    8.27     9.39  1 h. 12 min.
2    15806 REGIONAL   10.20    11.39  1 h. 19 min.
3    15808 REGIONAL   11.43    13.09  1 h. 26 min.
4          15070 MD   12.27    13.39  1 h. 12 min.
5    15810 REGIONAL   13.33    14.52  1 h. 19 min.
6          15090 MD   14.27    15.39  1 h. 12 min.
7    15812 REGIONAL   15.39    17.09  1 h. 30 min.
8          15072 MD   16.57    18.09  1 h. 12 min.
9    15814 REGIONAL   17.39    18.55  1 h. 16 min.
10         15018 MD   18.06    19.18  1 h. 12 min.
11         15918 MD   18.57    20.09  1 h. 12 min.
12         15074 MD   19.45    20.55  1 h. 10 min.
13   15848 REGIONAL   20.01    21.09   1 h. 8 min.
14   15850 REGIONAL   20.59    22.19  1 h. 20 min.
15   15822 REGIONAL   21.38    22.55  1 h. 17 min.
==================================================
```

### **Changing default origin and/or destination stations**
In order to change default timetable stations you just need to add `-u` flag in the CLI, and next time you won't need to add the used `-o` and `-t` params:

```
$ renfe-cli -o MADRI -t BARCE -u
```

If changing defaults, a file is created under user's home directory in file `~/.renfe_default_stations.json`


---

Issues
------
If Renfe's website is changed or you find any issue or enhancements, please: [create an issue](https://github.com/gerardcl/renfe-cli/issues)


Installation alternatives (getting latest source code)
------------------------------------------------------
If you want to install latest source code:
```
$ pip install git+http://github.com/gerardcl/renfe-cli
```
or
```
$ git clone git://github.com/gerardcl/renfe-cli
$ cd renfe-cli
$ python setup.py install
```
