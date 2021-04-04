#!/bin/bash

# Assembler flags were copied from a comment in 6502_functional_test.as65.
#
# The -s2 flag has been removed to cause assembler to produce a binary.
#
# Note that the assembler is a 32-bit executable, and therefore requires a
# 32-bit glibc and libstdc++ to be installed on 64-bit systems.
#
# To install on Fedora 33: sudo dnf install glibc.i686 libstdc++.i686

./as65 -l -m -w -h0 6502_functional_test.a65
