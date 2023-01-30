#!/bin/bash

docker build -t axum-sandbox .
docker run -d -p 3000:3000 axum-sandbox
