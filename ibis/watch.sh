#!/bin/bash

render() {
  dot -Tpng -o /tmp/last.png && kitty +kitten icat --align=left /tmp/last.png
}

always_render() {
  while read -r line;
  do
    echo "$line" | render
  done
}

cargo watch -q -x run | always_render
