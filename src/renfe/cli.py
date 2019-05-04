#!/usr/bin/env python
import optparse
import logging
from datetime import date

LOGGING_LEVELS = {'critical': logging.CRITICAL,
                  'error': logging.ERROR,
                  'warning': logging.WARNING,
                  'info': print,
                  'debug': logging.DEBUG}

def main():

  # defaults
  stations = {'SILS':'79202','BCN':'BARCE'}
  today = date.today()

  # opt parser
  p = optparse.OptionParser()
  p.add_option('--year', '-y', default=today.year, help='Year selected to get the timetable from')
  p.add_option('--month', '-m', default=today.month, help='Month of the year to get the timetable from')
  p.add_option('--day', '-d', default=today.day, help='Day of the month to get the timetable from')
  p.add_option('--origin', '-o', default=stations.get('SILS'), help='From/Origin ID of the train station. Use flag \'-s <possible station name>\' in order to search for IDs')
  p.add_option('--to', '-t', default=stations.get('BCN'), help='To/Destination ID of the train station. Use flag \'-s <possible station name>\' in order to search for IDs')
  p.add_option('--search', '-s', default='', help='You need to get the stations IDs, searching by names; in order to apply right inputs for origins and/or destinations')
  p.add_option('--logging-level', '-l', help='Logging level')
  p.add_option('--logging-file', '-f', help='Logging file name')

  options, arguments = p.parse_args()

  # Logging defaults to warning: critical, error and warning messages.
  logging_level = LOGGING_LEVELS.get(options.logging_level, logging.NOTSET)
  logging.basicConfig(level=logging_level, filename=options.logging_file,
                      format='%(asctime)s %(levelname)s: %(message)s',
                      datefmt='%Y-%m-%d %H:%M:%S')

  print("Today is: {}".format(today))

  # print timetable for given origin and to stations for a given date
  if options.search == '':
    import warnings
    warnings.filterwarnings("ignore", message="numpy.dtype size changed")
    warnings.filterwarnings("ignore", message="numpy.ufunc size changed")
    import pandas as pd

    if len(str(options.month)) == 1:
      options.month = '0' + str(options.month)
    if len(str(options.day)) == 1:
      options.day = '0' + str(options.day)

    print("Searching timetable for date: {}-{}-{}".format(options.year, options.month, options.day))
    print("From {} to {}".format(options.origin, options.to))

    urlTimetable = 'http://horarios.renfe.com/HIRRenfeWeb/buscar.do?O={}&D={}&ID=s&AF={}&MF={}&DF={}'.format(options.origin, options.to, options.year, options.month, options.day)

    logging.debug(urlTimetable)

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

    class RenfeHTMLParser(HTMLParser):
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

    parser = RenfeHTMLParser()
    parser.feed(data)
    parser.close()

    stationsWithID = dict(zip(parser.data, parser.links))

    for key in stationsWithID:
      if options.search.lower() in key.lower():
          print(key, stationsWithID.get(key))

if __name__ == '__main__':
    main()
