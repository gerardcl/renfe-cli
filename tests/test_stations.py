# TODO get more tests!!!
# test_input = (
#     # search ok
#     ('-s sil', None), ('-s barc', None),

#     # search nook
#     ('-s 123', None),

#     # default
#     ('', None),

#     # change default
#     ('-o 79300 -u', None),

#     # new default
#     ('', None),

#     # same as default
#     ('-d 0 -o 79202 -t BARCE', None),

#     # wrong inputs
#     # ('-d notanumber', SystemExit), ('-o BARCE -t BARCE', SystemExit), ('-d -1', SystemExit)
# )
import pytest
from renfe.stations import get_stations, get_station_name, station_exists, get_station_and_key
from renfe.utils import RenfeException

def test_renfe_cli_should_get_list_of_stations_objects():
    stations = get_stations()

    assert type(stations) == list
    assert any(stations)


def test_renfe_cli_should_get_name_of_station_from_id():
    name = get_station_name("79202")

    assert name == "SILS"


def test_renfe_cli_should_raise_exception_if_id_does_not_exists():
    with pytest.raises(RenfeException):
        get_station_name("123")


def test_renfe_cli_should_find_station_by_id():
    exists = station_exists("79202")

    assert exists == True


def test_renfe_cli_should_return_false_when_station_by_id_not_found():
    exists = station_exists("123")

    assert exists == False


def test_renfe_cli_should_return_stations_infos_when_key_is_found():
    stations_infos = get_station_and_key("sil")

    assert len(stations_infos) > 0


def test_renfe_cli_should_return_empty_when_station_by_id_not_found():
    stations_infos = get_station_and_key("123")

    assert len(stations_infos) == 0
