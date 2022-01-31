#!/bin/bash

render() {
  sleep 1
  dot -Tpng -o ./last.png >> watch.log 2>&1
  kitty +kitten icat --align=left ./last.png >> watch.log 2>&1
}

always_render() {
  while read -r line;
  do
    echo "$line" | render
  done
}

watcher () {
  cargo watch -q -x run | always_render &
}

watcher &
echo "open http://localhost/watch.html to view the output"
python3 -m http.server &> http.server.log
