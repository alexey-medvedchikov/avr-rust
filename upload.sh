#!/usr/bin/env bash

set -euo pipefail

ELF_FILE=$1

avr-objcopy -j .text -j .data -O ihex "$ELF_FILE" object.hex
avr-objcopy -j .eeprom --set-section-flags=.eeprom=alloc,load \
	--change-section-lma .eeprom=0 --no-change-warnings -O ihex "$ELF_FILE" eeprom.hex
avr-size object.hex
avr-size eeprom.hex
avrdude -p m168 -c usbasp -P usb \
	-U flash:w:object.hex \
	-U eeprom:w:eeprom.hex \
	-U lfuse:w:0xE2:m \
	-U hfuse:w:0xDF:m
