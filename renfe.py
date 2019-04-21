#!/usr/bin/env python
import optparse
from datetime import date
import warnings
warnings.filterwarnings("ignore", message="numpy.dtype size changed")
warnings.filterwarnings("ignore", message="numpy.ufunc size changed")
import pandas as pd

stations = {'SILS':'79202','BCN':'BARCE'}
today = date.today()

p = optparse.OptionParser()
p.add_option('--year', '-y', default=today.year)
p.add_option('--month', '-m', default=today.month)
p.add_option('--day', '-d', default=today.day)
p.add_option('--origin', '-o', default="SILS")
p.add_option('--to', '-t', default="BCN")

options, arguments = p.parse_args()

print("Today is: {}".format(today))
print("Searching timetable for date: {}-{}-{}".format(options.year, options.month, options.day))
print("From {} to {}".format(options.origin, options.to))

url = 'http://horarios.renfe.com/HIRRenfeWeb/buscar.do?O={}&D={}&ID=s&AF={}&MF={}&DF={}'.format(stations.get(options.origin, '79202'), stations.get(options.to, 'BARCE'), options.year, options.month, options.day)

tables = pd.read_html(url) # Returns list of all tables on page
timetable = tables[4] # Select table of interest

print(timetable)




# OLD - manual implementatation after pandas discovery

# import urllib.request

# web = urllib.request.urlopen(url)
# content =  web.read().decode(web.headers.get_content_charset())

# import re

# data = re.subn(r'<(html).*?</\1>(?s)', '', content)[0]

# from html.parser import HTMLParser

# class MyHTMLParser(HTMLParser):

#   def __init__(self):
#     HTMLParser.__init__(self)
#     self.recording = 0
#     self.data = []

#   def handle_starttag(self, tag, attrs):
#     if tag == 'table':
#       for name, value in attrs:
#         if name == 'class' and value == 'txt_borde1':
#           print(name, value)
#           print("Encountered the beginning of a %s tag" % tag)
#           self.recording = 1

#   def handle_endtag(self, tag):
#     if tag == 'table':
#       self.recording -=1
#       print("Encountered the end of a %s tag" % tag)

#   def handle_data(self, data):
#     if self.recording and re.sub('\s+', '', data) != '':
#       self.data.append(re.sub('\s+', '', data))

# parser = MyHTMLParser()

# doc = parser.feed(data)


# parser.close()

# print(parser.data)
