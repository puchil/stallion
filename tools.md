# Tools needed for Minix OS to become a desktop OS

## Type - Type of tool
1. Default - Part of Desktop Environment
1. Basic - Comes with the basic install of OS
1. Productivity - Productivity Tools (Office, 
1. Developer - All Developer tools needed

## Productivity
Name         | Description | Type
------------ | ------------------------------ | ---------------
Notes | Take quick notes | Basic
Text Editor (vi) | Edit Text files/check log files | Default

## Admin
Name         | Description | Type
------------ | ------------------------------ | ---------------
Disk Manager | Windows: Disk Management or gparted | Basic
Processes (ps) | Processes running in the system | Default

## Files - eq of LS Command
List information about the files in the home directory of the user. Takes features from ls/cd
* Show Hidden files (-A/--almost-all)/do not ignore entries starting with ., but do not list implied . and ..
* Show Backups (-B, --ignore-backups)/list implied entries ending with ~
* list directories themselves, not their contents (-d, --directory)
