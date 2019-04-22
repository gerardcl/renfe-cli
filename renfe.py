#!/usr/bin/env python
import optparse
from datetime import date
import warnings
warnings.filterwarnings("ignore", message="numpy.dtype size changed")
warnings.filterwarnings("ignore", message="numpy.ufunc size changed")
import pandas as pd

stations = {'SILS':'79202','BCN':'BARCE'}
today = date.today()

p = optparse.OptionParser()
p.add_option('--year', '-y', default=today.year)
p.add_option('--month', '-m', default=today.month)
p.add_option('--day', '-d', default=today.day)
p.add_option('--origin', '-o', default="SILS")
p.add_option('--to', '-t', default="BCN")

options, arguments = p.parse_args()

print("Today is: {}".format(today))
print("Searching timetable for date: {}-{}-{}".format(options.year, options.month, options.day))
print("From {} to {}".format(options.origin, options.to))

url = 'http://horarios.renfe.com/HIRRenfeWeb/buscar.do?O={}&D={}&ID=s&AF={}&MF={}&DF={}'.format(stations.get(options.origin, '79202'), stations.get(options.to, 'BARCE'), options.year, options.month, options.day)

tables = pd.read_html(url) # Returns list of all tables on page
timetable = tables[4] # Select table of interest
timetable = timetable.drop(timetable.columns[[4, 5, 6]], axis=1)

print(timetable)
