import requests
import json
from functools import lru_cache
from typing import List

from renfe.utils import RenfeException


@lru_cache(maxsize=32)
def get_stations():
    stations_js = requests.get(
        'https://www.renfe.com/content/dam/renfe/es/General/buscadores/javascript/estacionesEstaticas.js')
    if stations_js.status_code != 200 or stations_js.text.strip() == "":
        raise RenfeException("Looks like renfe web site is down? or maybe something was changed?")
    stations = json.loads(stations_js.text.split('=')[1].strip(';'))

    return stations


def get_station_and_key(search: str) -> List[str]:
    stations_infos = []
    try:
        for station in get_stations():
            if search.lower() in station['desgEstacion'].lower():
                stations_infos.append(f"{station['desgEstacion']}: {station['cdgoEstacion']}")
    except Exception as ex:
        raise RenfeException(ex)

    return stations_infos


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
