#!/bin/bash

TARGET=${1:-"x86_64-unknown-linux-gnu"}
SCRIPT_PATH="$( cd -- "$(dirname "$0")" >/dev/null 2>&1 ; pwd -P )"
DIRNAME=$(dirname $(dirname "$SCRIPT_PATH"))
APP_DIR="/app"
CONTAINER_NAME="ccv-builder"

if [ "$(expr substr $(uname -s) 1 5)" == "MINGW" ]; then
    DIRNAME="/$DIRNAME"
    APP_DIR="/$APP_DIR"
fi

docker run --name $CONTAINER_NAME -e TARGET=$TARGET -v $DIRNAME:$APP_DIR ccv_builder:local 

CONTAINER=$(docker ps --filter name=$CONTAINER_NAME -a -q)

docker kill $CONTAINER
docker rm $CONTAINER