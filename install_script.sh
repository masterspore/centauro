#!/bin/bash

cargo build
mkdir ~/centauro_server
sudo cp config.ini ~/centauro_server
sudo cp -r public ~/centauro_server
sudo cp target/debug/centauro_server ~/centauro_server
