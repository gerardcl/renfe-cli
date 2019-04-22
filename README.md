RENFE TIMETABLES CLI
====================

Usage:
```
./renfe.py -h
Usage: renfe.py [options]

Options:
  -h, --help            show this help message and exit
  -y YEAR, --year=YEAR          -- timetable's year
  -m MONTH, --month=MONTH       -- timetable's month
  -d DAY, --day=DAY             -- timetable's day
  -o ORIGIN, --origin=ORIGIN    -- timetable's origin station ID
  -t TO, --to=TO                -- timetable's destination stations ID
  -s SEARCH, --search=SEARCH    -- search for IDs related to train station names
```
**Note 1**: if not searching for IDs, timetable defaults to `today`, from `Sils` to `Barcelona`

**Note 2**: when using search functionality, it will provide you with the IDs (to use as an origin or destiation train station) of the stations that are similar to the input text to search. Example:

```
./renfe.py -s sil
Today is: 2019-04-22
Searching stations like: sil
SANESTEVODOSIL 22003
SANPEDRODOSIL 22004
SILLA 64200
SILS 79202
```