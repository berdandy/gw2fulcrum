#!/bin/bash

echo NOTE: USE $0 --reset TO GET FRESH DATA FROM API

if [ "$1" == "--reset" ]; then
  gw2search --reset-data
fi

echo -n Generating offline caches from latest api

CACHES="skill trait"

for cache in $CACHES; do
	echo -n "."
	gw2search --csv --$cache | grep -v ',""$' > src/${cache}s.csv
done

echo " done"
