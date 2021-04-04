#!/bin/bash

# For some reason, the output of the assembler is missing 10 bytes of leading
# zeros compared to the prebuilt binaries included in the repo. Since we need
# to rebuild the binaries in order to disable decimal mode tests, we can work
# around this by prepending 10 bytes of zeros to the generated binary.

cat <(head -c 10 /dev/zero) 6502_functional_test.bin > 6502_functional_test_padded.bin

