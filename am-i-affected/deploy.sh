#!/bin/sh
set -xe

yarn install
yarn build
rm -rf ../build
mv build ../
cd ..
git stash
git checkout gh-pages
rm -rf static
mv build/* .
rmdir build
git add .
git commit -m "Update"
git push
git checkout master
cd am-i-affected
git stash pop
