#!/bin/bash

npm install ./frontend

docker build -t Architecture-server ./backend
