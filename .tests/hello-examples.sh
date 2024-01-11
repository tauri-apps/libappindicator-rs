#!/bin/bash
set -e

readonly workdir=$(dirname $(dirname $(realpath $0)))
cd ${workdir}

echo "DISPLAY=${DISPLAY:-""}"
if [ ! -d /tmp/.X11-unix ]; then
    echo "Directory /tmp/.X11-unix does not exist."
fi

xhost + # Note this disable access control on Xorg, letting containers connect.
docker-compose -f .tests/docker-compose.yml up --build || true
xhost - # This reenables access control
