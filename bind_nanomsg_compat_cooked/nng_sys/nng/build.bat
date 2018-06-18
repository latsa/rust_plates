@echo off

md build
cd build

cmake ..
cmake --build . --config Debug
ctest -C Debug .
cmake --build . --config Debug --target install

:cmake -G Ninja ..
:ninja
:ninja test