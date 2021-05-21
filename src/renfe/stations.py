import requests
import json
import colorama
from functools import lru_cache

from renfe.utils import RenfeException

stations_js = requests.get('https://www.renfe.com/content/dam/renfe/es/General/buscadores/javascript/estacionesEstaticas.js')
stations = json.loads(stations_js.text.split('=')[1].strip(';'))


@lru_cache
def get_stations():
    return stations


def get_station_by_key(search: str):
    found_any = False
    print(colorama.Fore.GREEN + "Searching stations like: {}".format(search))
    for station in get_stations():
        if search.lower() in station['desgEstacion'].lower():
            found_any = True
            print(colorama.Fore.GREEN + f"{station['desgEstacion']}: {station['cdgoEstacion']}")
    
    if not found_any:
        print(colorama.Fore.RED + "Sorry, no stations found for input value: {}".format(search) + colorama.Fore.RESET)


def get_station_name(id: str) -> str:
    for station in get_stations():
        if id == station['cdgoEstacion']:
            return station['desgEstacion']
    raise RenfeException(f"Station id {id} not found!")
    

def station_exists(id: str) -> bool:
    for station in get_stations():
        if id == station['cdgoEstacion']:
            return True
    return False
