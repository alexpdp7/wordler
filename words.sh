#!/bin/bash

LANG=ascii grep -E '^[a-z]{5}$' /usr/share/dict/american-english >words
