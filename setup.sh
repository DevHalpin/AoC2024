#!/bin/bash
for i in {0..9}; do
  if [ $i -gt 0 ]; then
    mkdir -p "day-0$i"
    cp -a starter/. day-0$i/
  fi

  mkdir -p "day-1$i"
  cp -a starter/. day-1$i/

  if [ $i -lt 6 ]; then
    mkdir -p "day-2$i"
    cp -a starter/. day-2$i/
  fi
done