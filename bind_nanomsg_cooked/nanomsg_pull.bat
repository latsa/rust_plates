@echo off
:set mode="pull"
:.\target\debug\nanomsg_cooked
.\target\debug\nanomsg_cooked pull
pause

