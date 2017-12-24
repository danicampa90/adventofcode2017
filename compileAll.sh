#!/bin/bash
(cd day1 && cargo test && cd ..) || exit 1
(cd day2 && cargo test && cd ..) || exit 1
(cd day3 && cargo test && cd ..) || exit 1
(cd day4 && cargo test && cd ..) || exit 1
(cd day5 && cargo test && cd ..) || exit 1
(cd day6 && cargo test && cd ..) || exit 1