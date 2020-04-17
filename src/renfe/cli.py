#!/usr/bin/env python
import optparse
import logging
import colorama
from datetime import date

from renfe.interface import get_time_table, search_stations_ids
from renfe.utils import RenfeException, ConfigurationMgmt


def main():
    # defaults
    config = ConfigurationMgmt()
    stations = config.get_config()
    today = date.today()

    # opt parser
    p = optparse.OptionParser()
    p.add_option('--year', '-y', default=today.year, help='year selected to get the timetable from')
    p.add_option('--month', '-m', default=today.month, help='month of the year to get the timetable from')
    p.add_option('--day', '-d', default=today.day, help='day of the month to get the timetable from')
    p.add_option('--origin', '-o', default=stations.get('origin', '79202'),
                 help='from/origin ID of the train station. Use flag '
                      '\'-s <possible station name>\' in order to search for IDs')
    p.add_option('--to', '-t', default=stations.get('to', 'BARCE'),
                 help='to/destination ID of the train station. Use flag '
                      '\'-s <possible station name>\' in order to search for IDs')
    p.add_option('--search', '-s', default='',
                 help='you need to get the stations IDs, searching by names; '
                      'in order to apply right inputs for origins and/or destinations')
    p.add_option('--logging-level', '-l',
                 help='logging level defaults to warning and possible values are:'
                      ' debug, info, warning, error and critical')
    p.add_option('--logging-file', '-f',
                 help='logging file name is required if you want to submit an issue with more information')
    p.add_option('--update-config', '-u', default=False, action='store_true', dest='update_config',
                 help='change your origin and destination stations to defaults when loading this flag')
    options, arguments = p.parse_args()

    # check and configure input parameters
    try:
        config.configs_and_checks(options)
        if options.update_config:
            config.set_config({'origin': options.origin, 'to': options.to})
    except RenfeException as err:
        logging.error(err)
        logging.error(
            "Error configuring.. Check your inputs and enable debug. If the problem persists,"
            " create an issue at http://www.github.com/gerardcl/renfe-cli/issues")
        exit(1)

    colorama.init(autoreset=True)
    print(colorama.Fore.GREEN + "Today is: {}".format(today) + colorama.Fore.RESET)

    # print timetable for given origin and to stations for a given date
    if options.search == '':
        try:
            get_time_table(options.origin, options.to, options.year, options.month, options.day)
        except RenfeException as err:
            logging.error(err)
            logging.error(
                "No timetables found... Check your inputs and enable debug. If the problem persists,"
                " create an issue at http://www.github.com/gerardcl/renfe-cli/issues")
            exit(1)

    # search into list of Station names and its Renfe identifiers
    else:
        try:
            search_stations_ids(options.search)
        except RenfeException as err:
            logging.error(err)
            logging.error(
                "Error searching stations IDs... Check your inputs and enable debug. If the problem persists,"
                " create an issue at http://www.github.com/gerardcl/renfe-cli/issues")
            exit(1)


if __name__ == '__main__':
    main()
