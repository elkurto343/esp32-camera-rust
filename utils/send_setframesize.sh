#!/bin/bash

# TODO: replace with rust cli tool

if [ -z "$BOARD_IP" ]; then
	echo "Error: BOARD_IP not set"
	exit 1
fi

# Check if FRAME_SIZE is provided as arg
if [ "$#" -ne 1 ]; then
	echo "Usage: ./script.sh <FRAME_SIZE>"
	exit 1
fi

FRAME_SIZE="$1"

# Convert FRAME_SIZE to big-endian 4-byte hexadecimal
FRAME_SIZE_HEX=$(printf '%08x' "$FRAME_SIZE" | sed 's/\(..\)/\\x\1/g')

echo "$FRAME_SIZE_HEX"

# Send the command to the device
printf "\x03${FRAME_SIZE_HEX}" | nc "$BOARD_IP" 8080
