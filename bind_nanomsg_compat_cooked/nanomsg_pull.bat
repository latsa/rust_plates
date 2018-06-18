@echo off
:set mode="pull"
:.\target\debug\nanomsg_compat_cooked
.\target\debug\nanomsg_compat_cooked pull
pause

