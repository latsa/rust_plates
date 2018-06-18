@echo off
call "C:\Program Files (x86)\Microsoft Visual Studio 14.0\VC\vcvarsall.bat" x86

md build
cd build
cmake -DNN_STATIC_LIB=ON -DNN_ENABLE_DOC=OFF ..
cmake --build . --config Debug
cmake --build . --config Release
ctest -C Debug .
ctest -C Release .
cmake --build . --config Release --target install
