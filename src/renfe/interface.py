import sys
import colorama
import logging
import ssl
import urllib.request

from renfe.utils import RenfeException, RenfeHTMLParser

def get_time_table(origin, to, year, month, day):
  try:
    import warnings
    warnings.filterwarnings("ignore", message="numpy.dtype size changed")
    warnings.filterwarnings("ignore", message="numpy.ufunc size changed")
    import pandas as pd

    # nice input values for month and day
    if len(str(month)) == 1:
      month = '0' + str(month)
    if len(str(day)) == 1:
      day = '0' + str(day)

    print(colorama.Fore.GREEN + "Searching timetable for date: {}-{}-{}".format(year, month, day) + colorama.Fore.RESET)
    print(colorama.Fore.GREEN + "From {} to {}".format(origin, to) + colorama.Fore.RESET)

    url_timetable = 'http://horarios.renfe.com/HIRRenfeWeb/buscar.do?O={}&D={}&ID=s&AF={}&MF={}&DF={}'.format(origin, to, year, month, day)

    logging.debug(url_timetable)

    # get timetable
    web = urllib.request.urlopen(url_timetable, context=ssl._create_unverified_context())
    tables = pd.read_html(web) # Returns list of all tables on page
    timetable = tables[4] # Select table of interest
    timetable = timetable.drop(timetable.columns[[4, 5, 6]], axis=1) # Remove not required columns
    # show timetable
    print(colorama.Fore.GREEN + "=================== TIMETABLE ====================" + colorama.Fore.RESET)
    print(timetable)
    print(colorama.Fore.GREEN + "==================================================" + colorama.Fore.RESET)
  except:
    logging.debug(sys.exc_info())
    raise RenfeException("An error occurred while getting timetable")

def search_stations_ids(search):
  try:
    import re
    print(colorama.Fore.GREEN + "Searching stations like: {}".format(search) + colorama.Fore.RESET)

    url_stations = 'http://horarios.renfe.com/HIRRenfeWeb/estaciones.do?&ID=s&icid=VTodaslasEstaciones'

    logging.debug(url_stations)

    found_any = False

    web = urllib.request.urlopen(url_stations, context=ssl._create_unverified_context())
    content =  web.read().decode(web.headers.get_content_charset())
    data = re.subn('', content,r'<(html).*?</\1>(?s)')[0]

    parser = RenfeHTMLParser()
    parser.feed(data)
    parser.close()

    stations_with_id = dict(zip(parser.data, parser.links))

    # show findings
    for key in stations_with_id:
      if search.lower() in key.lower():
        found_any = True
        print(key +": "+ stations_with_id.get(key))

    if not found_any:
      print("Sorry, no stations found for input value: {}".format(search))
  except:
    logging.debug(sys.exc_info())
    raise RenfeException("An error occurred while searching for stations IDs")
