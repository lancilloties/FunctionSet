#!/bin/bash
echo "Cmdline:";
/bin/cat /proc/self/cmdline;
echo
echo "Comm:";
/bin/cat /proc/self/comm;
echo
echo "Status:";
/bin/cat /proc/self/status;
