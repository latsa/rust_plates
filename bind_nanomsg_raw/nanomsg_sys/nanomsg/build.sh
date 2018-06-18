#! /bin/bash

mkdir build
cd build
cmake -DNN_STATIC_LIB=ON -DNN_ENABLE_DOC=OFF -DNN_DISABLE_GETADDRINFO_A=OFF ..
cmake --build . --config debug
cmake --build . --config release
ctest -C debug .
ctest -C release .
sudo cmake --build . --config release --target install
sudo ldconfig


