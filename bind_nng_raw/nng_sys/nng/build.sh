#! /bin/bash

mkdir build
cd build

#cmake ..
#cmake --build .
#ctest .
#sudo cmake --build . --target install

cmake -G Ninja ..
ninja
ninja test
sudo cp  ./libnng.a /usr/local/lib
sudo ldconfig
