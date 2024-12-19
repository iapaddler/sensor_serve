# Sensor Serve

## Introduction
Sensor serve provides an interface to a Bosch BMP388 sensor. The server is written in Rust. The sensor provides measurements for atmospheric pressure and temperature. It uses the Bosch Sensor API (https://github.com/boschsensortec/BMP3_SensorAPI). The Bosch code is done in C. Therefore, sensor_serve is Rust code that calls a C code function to take sensor measurements.

## Server
The server takes sensor measurements at an interval. Refer to the constant PERIOD. Measurements are written to a local file, sensor.dat. The latest 100 measurements are saved in sensor.dat. The max and min values are calculated and included in the data set.

## Sensor
The sensor uses the I2C bus (https://www.i2c-bus.org/). There are 2 modes of operation supported by the Bosch device, manual and normal. The manual mode is easiest to configure, supporting one measurement. The device must be reset and configured again to take further measurements. Normal mode is capable of taking repeated measurements at configured intervals. The configuration of normal mode includes filter settings that define intervals among other settings.

The sensor requires calibration. The details of calibration are beyond the scope of this document. It can get complicated. This is part of the rationale behind using the Bosch code to take measurements. 

## Driver
The Bosch code is based on the COINES hardware platform. COINES allows users to evaluate sensors using the Bosch Sensortec Application Board. The driver for this sensor serve replaces the COINES code with an I2C interface, supporting the BMP388 device.

The driver uses the Linux I2C interface (https://www.kernel.org/doc/Documentation/i2c/dev-interface). It uses an ioctl to access the kernel driver. The buffer detail is defined in linux/i2c-dev.h.

## Clone
This repo uses a git sub module for utility functions. Use this command to clone both the repo and the sub module (assuming ssh access):
git clone --recurse-submodules git@github.com:iapaddler/utils.git

## Hardware
The project has been tested on a pi5 platform. This uses an ARM processor. The sensor is wired into the GPIO pins. In order to test basic functionality on an X64 platform without the sensor set the constant X86 to 1. It defaults to 0.

## Build
There are 2 steps to build sensor serve. 1) The driver and 2) the server.
### Driver Build
* Get rserve code (https://github.com/iapaddler/rserve)
* `cd rserve/bmp388/`
* `make`

### Server build
#### Install Rust if needed
* `$ curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh`
#### Build server code
* `cd sensor_serve`
* `cargo build`
