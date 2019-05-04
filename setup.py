import codecs
import os

from setuptools import setup, find_packages

def read(*parts):
    return codecs.open(os.path.join(os.path.abspath(os.path.dirname(__file__)), *parts), 'r').read()

long_description = read('README.md')

setup(
    name='renfe-cli',
    version='1.0.2',
    description='Get faster RENFE Spanish Trains timetables in your terminal',
    long_description=long_description,
    keywords='Get faster RENFE Spanish Trains timetables terminal',
    author='Gerard Castillo',
    author_email='gerardcl@gmail.com',
    url='https://github.com/gerardcl/renfe-cli',
    license='BSD',
    packages = find_packages('src'),
    package_dir = {'': 'src'},
    py_modules=['renfe-cli'],
    include_package_data=True,
    install_requires=[
        'setuptools', 'lxml', 'numpy', 'pandas', 'python-dateutil', 'pytz', 'six'
    ],
    entry_points="""
        [console_scripts]
        renfe-cli = renfe.cli:main
        """,
    classifiers=[
        'Development Status :: 5 - Production/Stable',
        'Environment :: Console',
        'Intended Audience :: End Users/Desktop',
        'Intended Audience :: Developers',
        'License :: OSI Approved :: BSD License',
        'Operating System :: MacOS',
        'Operating System :: Unix',
        'Operating System :: POSIX',
        'Operating System :: Microsoft :: Windows',
        'Programming Language :: Python',
        'Topic :: Utilities',
        'Topic :: Terminals',
        'Topic :: Text Processing :: Markup :: HTML',
    ],
    tests_require=['nose'],
    test_suite = 'nose.collector',
)
