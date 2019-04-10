#!/bin/bash

cargo build
gcc -std=gnu99 -o rusty_test app.c ./target/debug/librusttest.a
