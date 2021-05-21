import sys
import os
import json
import logging
from pathlib import Path


class RenfeException(Exception):
    def __init__(self, *args, **kwargs):
        Exception.__init__(self, *args, **kwargs)


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
                'origin': '79202',  # SILS
                'to': '71802'  # BCN PASSEIG DE GRACIA
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
        logging.basicConfig(level=self._logging_levels.get(options.logging_level, logging.WARN),
                            filename=options.logging_file,
                            format='%(asctime)s %(levelname)s: %(message)s',
                            datefmt='%Y-%m-%d %H:%M:%S')
        logging.debug(
            "params --> days: {}, origin: {}, destination: {}, search: {}, update_config:"
            " {}, logging_level: {}, logging_file: {} <--"
            .format(options.days, options.origin, options.to, options.search,
                    options.update_config, options.logging_level, options.logging_file))
        # Doing some checks
        if options.origin == options.to:
            raise RenfeException("Cannot search timetables if both origin and destiantion are the same")
        try:
            int(options.days)
            if options.days < 0:
                raise ValueError("Only today or future days can be searched...")
        except ValueError:
            logging.debug(sys.exc_info())
            raise RenfeException("Cannot search timetables if date params are not set with numbers")
