#!/usr/bin/env python
import optparse
import logging
import colorama
from datetime import date

from renfe.timetable import get_timetable
from renfe.stations import get_station_by_key, station_exists, get_station_name
from renfe.utils import RenfeException, ConfigurationMgmt


def main():
    # defaults
    config = ConfigurationMgmt()
    stations = config.get_config()
    today = date.today()

    # opt parser
    p = optparse.OptionParser()
    p.add_option('--days', '-d', default=0, help='number of days from today to get the timetable')
    p.add_option('--origin', '-o', default=stations.get('origin', '79202'),
                 help='from/origin ID of the train station. Use flag '
                      '\'-s <possible station name>\' in order to search for IDs')
    p.add_option('--to', '-t', default=stations.get('to', '71802'),
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
        if not (station_exists(options.origin) and station_exists(options.to)):
            logging.error(
                "Please, provide right values for origin and destination station names")
            exit(1)

        try:
            times = get_timetable(get_station_name(options.origin), get_station_name(options.to), int(options.days))
            print(colorama.Fore.GREEN + "=======================TIMETABLE======================")
            print(colorama.Fore.GREEN + " {:<10} | {:<10} | {:<10} | {:<10} ".format('Train','Departure','Arrival', 'Duration'))
            for time in times:
                print(colorama.Fore.GREEN + "------------------------------------------------------")
                print(colorama.Fore.GREEN + " {:<10} | {:<10} | {:<10} | {:<10} ".format(time[0],time[1],time[2], time[3]))
            print(colorama.Fore.GREEN + "======================================================" + colorama.Fore.RESET)
        except RenfeException as err:
            logging.error(err)
            logging.error(
                "No timetables found... Check your inputs and enable debug. If the problem persists,"
                " create an issue at http://www.github.com/gerardcl/renfe-cli/issues")
            exit(1)

    # search into list of Station names and its Renfe identifiers
    else:
        try:
            get_station_by_key(options.search)
        except RenfeException as err:
            logging.error(err)
            logging.error(
                "Error searching stations IDs... Check your inputs and enable debug. If the problem persists,"
                " create an issue at http://www.github.com/gerardcl/renfe-cli/issues")
            exit(1)


if __name__ == '__main__':
    main()
