#!/bin/bash

echo What day is it?

read DAY

echo Creating $DAY


FOLDER=day$DAY
cargo new $FOLDER
cp -r rust-template/src $FOLDER

touch $FOLDER/src/example.txt
touch $FOLDER/src/input.txt

cp rust-template/.gitignore $FOLDER
