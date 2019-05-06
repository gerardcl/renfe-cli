import sys
import re
import logging
from html.parser import HTMLParser

def configs_and_checks(options):
  LOGGING_LEVELS = {'critical': logging.CRITICAL,
                    'error': logging.ERROR,
                    'warning': logging.WARNING,
                    'info': logging.INFO,
                    'debug': logging.DEBUG}
  # Logging defaults to warning: critical, error and warning messages.
  logging.basicConfig(level=LOGGING_LEVELS.get(options.logging_level, logging.WARN), filename=options.logging_file,
                    format='%(asctime)s %(levelname)s: %(message)s',
                    datefmt='%Y-%m-%d %H:%M:%S')
  logging.debug("params --> year: {}, month: {}, day: {}, origin: {}, destination: {}, search: {}, logging level: {}, logging file: {} <--"
          .format(options.year, options.month, options.day, options.origin, options.to, options.search, options.logging_level, options.logging_file))
  # Doing some checks
  if options.origin == options.to:
    raise RenfeException("Cannot search timetables if both origin and destiantion are the same")
  try:
    int(options.year)
    int(options.month)
    int(options.day)
  except ValueError:
    logging.debug(sys.exc_info())
    raise RenfeException("Cannot search timetables if date params are not set with numbers")

class RenfeException(Exception):
  def __init__(self,*args,**kwargs):
    Exception.__init__(self,*args,**kwargs)

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

class ConfigurationMgmt():
  pass