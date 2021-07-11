#!/usr/bin/env bash

if [ "$PORT" ]; then
  export APP__HTTP_SERVER__PORT="$PORT"
fi

if [ "$DATABASE_URL" ]; then
  regex="^postgres:\/\/(.*):(.*)@(.*):([0-9]*)\/(.*)$"

  if [[ "$DATABASE_URL" =~ $regex ]]; then
    export APP__POSTGRES__USERNAME="${BASH_REMATCH[1]}"
    export APP__POSTGRES__PASSWORD="${BASH_REMATCH[2]}"
    export APP__POSTGRES__HOST="${BASH_REMATCH[3]}"
    export APP__POSTGRES__PORT="${BASH_REMATCH[4]}"
    export APP__POSTGRES__DATABASE_NAME="${BASH_REMATCH[5]}"
  else
    echo "Unable to parse environment variable DATABASE_URL"
  fi
fi

./home/appuser/rust-graphql-kata-001-web
