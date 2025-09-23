#!/bin/bash

docker-compose up -d
cargo sqlx migrate run
