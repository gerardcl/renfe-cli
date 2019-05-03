RENFE TIMETABLES CLI
====================

Get faster RENFE Spanish Trains timetables in your terminal.

- Installation:

Install python CLI package [renfe-cli](https://pypi.org/project/renfe-cli/)

```
$ pip install renfe-cli
```

- Usage:

```
$ renfe-cli -h
Usage: renfe-cli [options]

Options:
  -h, --help            show this help message and exit
  -y YEAR, --year=YEAR          -- timetable's year
  -m MONTH, --month=MONTH       -- timetable's month
  -d DAY, --day=DAY             -- timetable's day
  -o ORIGIN, --origin=ORIGIN    -- timetable's origin station ID
  -t TO, --to=TO                -- timetable's destination stations ID
  -s SEARCH, --search=SEARCH    -- search for IDs related to train station names
```

**Searching for IDs of train stations**
----

 When using search functionality, it will provide you with the IDs (to use as an origin or destiation train station) of the stations that are similar to the input text to search. Example:

```
$ renfe-cli -s sil
Today is: 2019-04-22
Searching stations like: sil
SANESTEVODOSIL 22003
SANPEDRODOSIL 22004
SILLA 64200
SILS 79202

$ renfe-cli -s barc
Today is: 2019-04-22
Searching stations like: barc
BARCELONA 97007
BARCELONA(TODAS) BARCE
BARCELONA-ARCDETRIOMF 78804
BARCELONA-PLAÇADECATALUNYA 78805
BARCELONA-TORREDELBARO 78801
BARCELOS 94024
BARCENA 14206
CEUTA-BARCO 99126
ELBARCENAL 05644
OBARCODEVALDEORRAS 20211
TANGERMED-BARCO 99124
TANGERVILLE-BARCO 99125
VILANOVADELABARCA 75102
```

**Getting the timetable**
----
Timetable defaults to `today`, from `Sils` (ID is `79202`) to `Barcelona` (ID is `BARCE`):

```
$ renfe-cli
Today is: 2019-04-22
Searching timetable for date: 2019-04-22
From 79202 to BARCE
   Tren / Recorrido  Salida  Llegada      Duración
0          15010 MD    6.22     7.39  1 h. 17 min.
1          15046 MD    6.37     7.54  1 h. 17 min.
2    25672 REGIONAL    6.49     8.39  1 h. 50 min.
3          15060 MD    7.23     8.39  1 h. 16 min.
4          15062 MD    7.52     9.09  1 h. 17 min.
5    15804 REGIONAL    8.20     9.39  1 h. 19 min.
6    25674 REGIONAL    8.49    10.40  1 h. 51 min.
7          15066 MD    9.08    10.20  1 h. 12 min.
8    15806 REGIONAL    9.50    11.09  1 h. 19 min.
9          15068 MD   10.27    11.39  1 h. 12 min.
10   25676 REGIONAL   10.49    12.41  1 h. 52 min.
11   15808 REGIONAL   11.50    13.09  1 h. 19 min.
12         15070 MD   12.27    13.39  1 h. 12 min.
13   25678 REGIONAL   12.49    14.39  1 h. 50 min.
14   15810 REGIONAL   13.50    15.09  1 h. 19 min.
15         15090 MD   14.27    15.39  1 h. 12 min.
16   25680 REGIONAL   14.49    16.40  1 h. 51 min.
17         15002 MD   14.57    16.09  1 h. 12 min.
18   15812 REGIONAL   15.50    17.09  1 h. 19 min.
19         15072 MD   16.27    17.39  1 h. 12 min.
20   15814 REGIONAL   17.20    18.39  1 h. 19 min.
21         15018 MD   17.57    19.09  1 h. 12 min.
22         15092 MD   18.27    19.39  1 h. 12 min.
23   15818 REGIONAL   18.50    20.09  1 h. 19 min.
24         15074 MD   19.27    20.39  1 h. 12 min.
25   15820 REGIONAL   20.20    21.39  1 h. 19 min.
26         15000 MD   21.07    22.19  1 h. 12 min.
27         15904 MD   21.27    22.39  1 h. 12 min.
```

Which would be the same as:

```
$ renfe-cli -y 2019 -m 4 -d 22 -o 79202 -t BARCE
Today is: 2019-04-22
Searching timetable for date: 2019-04-22
From 79202 to BARCE
   Tren / Recorrido  Salida  Llegada      Duración
0          15010 MD    6.22     7.39  1 h. 17 min.
1          15046 MD    6.37     7.54  1 h. 17 min.
2    25672 REGIONAL    6.49     8.39  1 h. 50 min.
3          15060 MD    7.23     8.39  1 h. 16 min.
4          15062 MD    7.52     9.09  1 h. 17 min.
5    15804 REGIONAL    8.20     9.39  1 h. 19 min.
6    25674 REGIONAL    8.49    10.40  1 h. 51 min.
7          15066 MD    9.08    10.20  1 h. 12 min.
8    15806 REGIONAL    9.50    11.09  1 h. 19 min.
9          15068 MD   10.27    11.39  1 h. 12 min.
10   25676 REGIONAL   10.49    12.41  1 h. 52 min.
11   15808 REGIONAL   11.50    13.09  1 h. 19 min.
12         15070 MD   12.27    13.39  1 h. 12 min.
13   25678 REGIONAL   12.49    14.39  1 h. 50 min.
14   15810 REGIONAL   13.50    15.09  1 h. 19 min.
15         15090 MD   14.27    15.39  1 h. 12 min.
16   25680 REGIONAL   14.49    16.40  1 h. 51 min.
17         15002 MD   14.57    16.09  1 h. 12 min.
18   15812 REGIONAL   15.50    17.09  1 h. 19 min.
19         15072 MD   16.27    17.39  1 h. 12 min.
20   15814 REGIONAL   17.20    18.39  1 h. 19 min.
21         15018 MD   17.57    19.09  1 h. 12 min.
22         15092 MD   18.27    19.39  1 h. 12 min.
23   15818 REGIONAL   18.50    20.09  1 h. 19 min.
24         15074 MD   19.27    20.39  1 h. 12 min.
25   15820 REGIONAL   20.20    21.39  1 h. 19 min.
26         15000 MD   21.07    22.19  1 h. 12 min.
27         15904 MD   21.27    22.39  1 h. 12 min.
```


---

Issues
----
If Renfe's website is changed please fire an issue in order to update the parsing methods and get it working again.


Installation alternatives (getting latest source code)
----
If you want to install latest source code:
```
$ pip install git+http://github.com/gerardcl/renfe-cli
```
or
```
$ git clone git://github.com/jsmits/github-cli.git
$ cd github-cli
$ python setup.py install
```
