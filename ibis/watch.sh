#!/bin/bash

_term() {
  echo "Caught SIGTERM signal: killing watchers"
  kill -TERM "$WATCH_PID" 2>/dev/null
}

trap _term SIGTERM

render() {
  dot -Tsvg -o ./watch/last.svg >> ./watch/watch.log 2>&1
}

always_render() {
  while read -r line;
  do
    rm -f ./watch/last.svg ./watch/last.dot
    echo "$line" > ./watch/last.raw.dot
    echo "$line" | dot > ./watch/last.dot
    echo "$line" | render
  done
}

spawn_watcher () {
  cargo watch -q -x run | always_render &
  WATCH_PID="$!"
}

server () {
  python3 -m http.server &> ./watch/http.server.log
}

spawn_watcher
echo "open http://localhost:8000/watch to view the output"
server
wait "$WATCH_PID"
