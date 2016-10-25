# tomato-blacklist-reducer
I wanted to use my home router to block some non-family friendly URLs. The blacklist I got is 500k 
big, but my router (running Tomato Shibby) only has 2048 bytes of regexes per rule. The program 
outputs the most often occuring snippets of strings from the blacklist after making sure they don't mask any URLs 
from a whitelist. The actual efficiency of the resulting filter is debatable, but the program works.

[![Clippy Linting Result](https://clippy.bashy.io/github/fralalonde/tomato-blacklist-reducer/master/badge.svg)](https://clippy.bashy.io/github/fralalonde/tomato-blacklist-reducer/master/log)
