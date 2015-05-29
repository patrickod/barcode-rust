# barcode-rust [![Build Status](https://travis-ci.org/patrickod/barcode-rust.svg?branch=master)](https://travis-ci.org/patrickod/barcode-rust)

Tiny binary to read scancodes from a Inateck BCST-20 barcode scanner which I
intend to use to catalogue the book collection at
[Noisebridge](https://noisebridge.net)

This project is mostly an experiment with rust and c libraries

## What

`scanner` is a small utility that can be used with a USB barcode scanner
to help the process of cataloging books at Noisebridge. It specifically
addresses two issues with using USB barcode scanners for this purpose

  a) The USB scanners emulate keyboards and so write out to STDIN. For
     systems with multiple input devices this is cumbersome
  b) We only want the consumer of this program's output to deal with
     valid ISBN codes to ingest them into the catalogue.

`scanner` uses
[libevdev](https://wiki.freedesktop.org/www/Software/libevdev/), a
wrapper around evdevices in Linux to "claim" the input device created by
the USB scanner such that its input events aren't consumable by other
programs accepting input from stdin at the same time. It filters parse
it's input for valid ISBNv13 barcodes. which it then writes to stdout
