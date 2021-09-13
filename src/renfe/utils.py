import sys
import os
import json
import logging
import optparse
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
        logging.debug(f"setting new defaults to {new_config}")
        self.__dict__ = new_config
        json.dump(self.__dict__, open(os.path.join(Path.home(), self._config_location), 'w'))

    def configs_and_checks(self, options):
        # Logging defaults to warning: critical, error and warning messages.
        logging.basicConfig(level=self._logging_levels.get(options.logging_level, logging.WARN),
                            filename=options.logging_file,
                            format='%(asctime)s %(levelname)s: %(message)s',
                            datefmt='%Y-%m-%d %H:%M:%S')
        logging.debug(
            "params --> browser: {}, days: {}, origin: {}, destination: {}, search: {}, update_config:"
            " {}, logging_level: {}, logging_file: {} <--"
            .format(options.browser, options.days, options.origin, options.to, options.search,
                    options.update_config, options.logging_level, options.logging_file))
        # Doing some checks
        if options.origin == options.to:
            raise RenfeException("Cannot search timetables if both origin and destiantion are the same")
        if options.browser not in ["firefox", "chrome"]:
            raise RenfeException("Only accepted browsers are: firefox or chrome")
        try:
            int(options.days)
            if int(options.days) < 0:
                raise ValueError("Only today or future days can be searched...")
        except ValueError:
            logging.debug(sys.exc_info())
            raise RenfeException("Cannot search timetables if date params are not set with numbers")

        # check and configure input parameters
        try:
            if options.update_config:
                self.set_config({'origin': options.origin, 'to': options.to})
        except RenfeException as err:
            logging.error(err)
            logging.error(
                "Error when handling configs... Check your inputs and enable debug. If the problem persists,"
                " create an issue at http://www.github.com/gerardcl/renfe-cli/issues")
            exit(1)


def parse_args(config):
    stations = config.get_config()

    p = optparse.OptionParser()
    p.add_option('--origin', '-o', default=stations.get('origin', '79202'),
                 help='from/origin ID of the train station. Use flag '
                      '\'-s <possible station name>\' in order to search for IDs')
    p.add_option('--to', '-t', default=stations.get('to', '71802'),
                 help='to/destination ID of the train station. Use flag '
                      '\'-s <possible station name>\' in order to search for IDs')
    p.add_option('--days', '-d', default=0, help='number of days from today to get the timetable (default: 0 - today)')
    p.add_option('--browser', '-b', default="firefox", help='possible browsers are "firefox" and "chrome" (default: firefox)')
    p.add_option('--search', '-s', default='',
                 help='you need to get the stations IDs, searching by names; '
                      'in order to apply right inputs for origins and/or destinations')
    p.add_option('--search-timeout', '-e', default=3,
                 help='search timeout in seconds (default to 3 seconds)')
    p.add_option('--logging-level', '-l',
                 help='logging level defaults to warning and possible values are:'
                      ' debug, info, warning, error and critical')
    p.add_option('--logging-file', '-f',
                 help='logging file name is required if you want to submit an issue with more information')
    p.add_option('--update-config', '-u', default=False, action='store_true', dest='update_config',
                 help='change your origin and destination stations to defaults when loading this flag')
    options, _ = p.parse_args()

    return options
