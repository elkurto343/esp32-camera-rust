#!/bin/bash

# TODO: replace with rust cli tool

if [ -z "$BOARD_IP" ]; then
	echo "Error: BOARD_IP not set"
	exit 1
fi

# Generate a filename with the timestamp
timestamp=$(date +"%Y%m%d_%H%M%S")
filename="${timestamp}.jpg"

# Send command to the device and save the response as a .jpg file
printf '\x01\x00\x00\x00\x00' | nc "$BOARD_IP" 8080 >"captures/$filename"
