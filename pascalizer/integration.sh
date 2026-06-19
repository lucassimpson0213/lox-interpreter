#! /usr/bin/bash

bun snake_to_pascal.test.ts

echo "TESTING SUITE"
echo "ensure all tests are fine"
echo "\n\n\n"

cp test/main.rs test/main.rs.bak
bun snake_to_pascal.ts test/main.rs


cat test/main.rs

rm test/main.rs
mv test/main.rs.bak test/main.rs





