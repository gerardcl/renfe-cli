import sys
import os
import re
import json
import logging
from pathlib import Path
from html.parser import HTMLParser

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
  _config_location = '.renfe_default_stations.json'
  _logging_levels = {'critical': logging.CRITICAL,
                    'error': logging.ERROR,
                    'warning': logging.WARNING,
                    'info': logging.INFO,
                    'debug': logging.DEBUG}

  def __init__(self):
    if os.path.exists(os.path.join(Path.home(), self._config_location)):
      self.__dict__ = json.load(open(os.path.join(Path.home(), self._config_location)))
    else:
      self.__dict__ = {
        'origin': '79202', # SILS
        'to': 'BARCE' # BCN
      }

  # get_defaults
  def get_config(self):
    return self.__dict__

  # set_defaults
  def set_config(self, new_config):
    logging.debug("setting new defaults to {}".format(new_config))
    self.__dict__ = new_config
    json.dump(self.__dict__, open(os.path.join(Path.home(), self._config_location), 'w'))

  def configs_and_checks(self, options):
    # Logging defaults to warning: critical, error and warning messages.
    logging.basicConfig(level=self._logging_levels.get(options.logging_level, logging.WARN), filename=options.logging_file,
                      format='%(asctime)s %(levelname)s: %(message)s',
                      datefmt='%Y-%m-%d %H:%M:%S')
    logging.debug("params --> year: {}, month: {}, day: {}, origin: {}, destination: {}, search: {}, update_config: {}, logging_level: {}, logging_file: {} <--"
            .format(options.year, options.month, options.day, options.origin, options.to, options.search, options.update_config, options.logging_level, options.logging_file))
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
