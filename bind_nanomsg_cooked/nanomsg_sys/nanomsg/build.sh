#! /bin/bash

mkdir build
cd build
cmake -DNN_STATIC_LIB=ON -DNN_ENABLE_DOC=OFF -DNN_DISABLE_GETADDRINFO_A ..
cmake --build .
ctest .
sudo cmake --build . --target install
sudo ldconfig


