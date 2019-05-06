import os
import sys
from datetime import date
from nose.tools import assert_raises
from renfe.cli import main

prog = 'renfe-cli'
today = date.today()


def test_commands():
  for cmd, exp in test_input:
    def check_command(cmd, exp):
      base = [prog]
      args = cmd.split(' ')
      if not args == ['']:  # need this for 'just `renfe-cli`' command test
        base.extend(args)
      sys.argv = base
      if type(exp) == type(Exception):
        assert_raises(exp, main)
      else:
        output = main()
        assert output == exp
    check_command.description = "command: %s %s" % (prog, cmd)
    yield check_command, cmd, exp


test_input = (
  # default
  ('', None),

  # same as default
  ('-y {} -m {} -d {} -o 79202 -t BARCE'.format(today.year, today.month, today.day), None),

  # search ok
  ('-s sil', None), ('-s barc', None),

  # search nook
  ('-s 123', None),

  # wrong inputs
  ('-d notanumber', SystemExit), ('-o BARCE -t BARCE', SystemExit), ('-m 30', SystemExit)
)
