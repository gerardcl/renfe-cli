import os
import colorama
from datetime import datetime, timedelta
from typing import List, Set
from bs4 import BeautifulSoup
from selenium import webdriver
from webdriver_manager.firefox import GeckoDriverManager

os.environ['WDM_LOG_LEVEL'] = '0'


def get_timetable(origin: str, destination: str, days_from_today: int = 0) -> List[Set]:
    print(colorama.Fore.GREEN + f"Searching timetable for date: {get_date(days_from_today)}")
    print(colorama.Fore.GREEN + "From {} to {}".format(origin, destination) + colorama.Fore.RESET)
    print(colorama.Fore.GREEN + "...this might take some seconds \
depending on the Renfe site speed..." + colorama.Fore.RESET)

    soup = get_soup(origin, destination, days_from_today)
    types = get_types(soup)
    durations = get_durations(soup)
    departures = get_departures(soup)
    arrivals = get_arrivals(soup)

    return list(zip(types, departures, arrivals, durations))


def get_soup(origin: str, destination: str, days_from_today: int) -> BeautifulSoup:
    firefox_options = webdriver.FirefoxOptions()
    firefox_options.set_headless()
    browser = webdriver.Firefox(executable_path=GeckoDriverManager().install(), firefox_options=firefox_options)
    browser.implicitly_wait(10)  # wait up to 10 seconds while trying to locate elements
    browser.get("https://www.renfe.com/es/es")

    origin_input = browser.find_element_by_css_selector("rf-awesomplete.rf-input-autocomplete:nth-child(1) \
> div:nth-child(1) > div:nth-child(2) > input:nth-child(1)")
    origin_input.send_keys(origin)
    origin_option = browser.find_element_by_css_selector("#awesomplete_list_1_item_0")
    origin_option.click()

    destination_input = browser.find_element_by_css_selector("rf-awesomplete.rf-input-autocomplete:nth-child(2) \
> div:nth-child(1) > div:nth-child(2) > input:nth-child(1)")
    destination_input.send_keys(destination)
    destination_option = browser.find_element_by_css_selector("#awesomplete_list_2_item_0")
    destination_option.click()

    time = browser.find_element_by_css_selector("div.rf-daterange__container-ipt:nth-child(2) > div:nth-child(2) \
> button:nth-child(2) > i:nth-child(1)")

    while days_from_today > 0:
        days_from_today = days_from_today - 1
        time.click()

    search_button = browser.find_element_by_css_selector("rf-button.rf-button")
    search_button.click()

    soup = BeautifulSoup(browser.page_source, 'html.parser')

    browser.quit()

    return soup


def get_departures(soup) -> List[str]:
    result = []
    attrs_departure = {
            'class': 'booking-list-element-big-font salida displace-text-xs',
        }
    departures = soup.find_all('div', attrs=attrs_departure)
    for departure in departures:
        result.append(departure.text.strip())
    return result


def get_arrivals(soup) -> List[str]:
    result = []
    attrs_arrival = {
            'class': 'booking-list-element-big-font llegada',
        }
    arrivals = soup.find_all('div', attrs=attrs_arrival)
    for arrival in arrivals:
        result.append(arrival.text.strip())
    return result


def get_durations(soup) -> List[str]:
    result = []
    attrs_duration = {
        'class': 'purple-font displace-text duracion hidden-xs',
        'aria-label': 'Duración'
    }
    durations = soup.find_all('div', attrs=attrs_duration)
    for duration in durations:
        result.append(duration.text.strip())
    return result


def get_types(soup) -> List[str]:
    result = []
    attrs_duration = {
            'class': 'purple-font displace-text duracion hidden-xs',
            'aria-label': 'Duración'
        }
    durations = soup.find_all('div', attrs=attrs_duration)
    attrs_duration_hidden = {
            'class': 'purple-font displace-text visible-xs text-nowrap',
        }
    durations_hidden = soup.find_all('div', attrs=attrs_duration_hidden)
    attrs_duration_and_type = {
            'class': 'displace-text',
        }
    durations_and_types = soup.find_all('div', attrs=attrs_duration_and_type)
    types = [item for item in durations_and_types if item not in durations_hidden + durations]
    for t in types:
        result.append(t.text.strip())
    return result


def get_date(days_from_today: int) -> str:
    day = datetime.today() + timedelta(days=days_from_today)
    return f"{day.year}-{day.month}-{day.day}"
