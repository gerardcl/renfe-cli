#!/usr/bin/env python
import optparse
from datetime import date
import warnings
warnings.filterwarnings("ignore", message="numpy.dtype size changed")
warnings.filterwarnings("ignore", message="numpy.ufunc size changed")
import pandas as pd

# defaults
stations = {'SILS':'79202','BCN':'BARCE'}
today = date.today()

# opt parser
p = optparse.OptionParser()
p.add_option('--year', '-y', default=today.year)
p.add_option('--month', '-m', default=today.month)
p.add_option('--day', '-d', default=today.day)
p.add_option('--origin', '-o', default="SILS")
p.add_option('--to', '-t', default="BCN")
p.add_option('--search', '-s', default='')

options, arguments = p.parse_args()

print("Today is: {}".format(today))

# print timetable for given origin and to stations for a given date
if options.search == '':
  print("Searching timetable for date: {}-{}-{}".format(options.year, options.month, options.day))
  print("From {} to {}".format(options.origin, options.to))

  urlTimetable = 'http://horarios.renfe.com/HIRRenfeWeb/buscar.do?O={}&D={}&ID=s&AF={}&MF={}&DF={}'.format(stations.get(options.origin, '79202'), stations.get(options.to, 'BARCE'), options.year, options.month, options.day)
  urlStations = 'http://horarios.renfe.com/HIRRenfeWeb/estaciones.do?&ID=s&icid=VTodaslasEstaciones'

  tables = pd.read_html(urlTimetable) # Returns list of all tables on page
  timetable = tables[4] # Select table of interest
  timetable = timetable.drop(timetable.columns[[4, 5, 6]], axis=1)

  print(timetable)

# search into list of Station names and its Renfe identifiers
else:
  print("Searching stations like: {}".format(options.search))
  import urllib.request

  urlStations = 'http://horarios.renfe.com/HIRRenfeWeb/estaciones.do?&ID=s&icid=VTodaslasEstaciones'

  web = urllib.request.urlopen(urlStations)
  content =  web.read().decode(web.headers.get_content_charset())

  import re

  data = re.subn(r'<(html).*?</\1>(?s)', '', content)[0]

  from html.parser import HTMLParser

  class MyHTMLParser(HTMLParser):
    def __init__(self):
      HTMLParser.__init__(self)
      self.recording = 0
      self.data = []
      self.links = []

    def handle_starttag(self, tag, attrs):
      if tag == 'a':
        attrs = dict(attrs)
      if tag == 'a' and attrs.get('class') == 'linkgrise':
        self.links.append(attrs['href'].split('\'')[1])
        self.recording = 1

    def handle_endtag(self, tag):
      if tag == 'a':
        self.recording = 0

    def handle_data(self, data):
      if self.recording and re.sub('\s+', '', data) != '' and len(data) > 1:
        self.data.append(re.sub('\s+', '', data))

  parser = MyHTMLParser()
  parser.feed(data)
  parser.close()

  stationsWithID = dict(zip(parser.data, parser.links))

  for key in stationsWithID:
    if options.search.lower() in key.lower():
        print(key, stationsWithID.get(key))
