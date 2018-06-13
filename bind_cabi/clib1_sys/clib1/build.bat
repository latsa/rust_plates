@echo off
call "C:\Program Files (x86)\Microsoft Visual Studio 14.0\VC\vcvarsall.bat" x86
cl -c square.c
lib square.obj /OUT:clib1.lib
del square.obj