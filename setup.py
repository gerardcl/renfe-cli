from setuptools import setup, find_packages

try:
    from pypandoc import convert_file
    read_md = lambda f: convert_file(f, 'rst')
except ImportError:
    print("warning: pypandoc module not found, could not convert Markdown to RST")
    read_md = lambda f: open(f, 'r').read()

setup(
    name='renfe-cli',
    version='2.1.0',
    description='Get faster RENFE Spanish Trains timetables in your terminal',
    long_description=read_md('README.md'),
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
        'setuptools<54.3.0',
        'lxml<=4.5.0',
        'numpy<1.18.5',
        'pandas<1.0.4',
        'python-dateutil<=2.8.1',
        'pytz<2020.2',
        'six<=1.14.0',
        'colorama<=0.4.3'
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
