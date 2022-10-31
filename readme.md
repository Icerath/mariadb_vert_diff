# MariaDB Vert Diff

Takes two files generated from output formatted with \G in mariadb.

Eg. select * from information_schema.system_variables\G

# Usage
    .mariadb_vert_diff[.exe] [start filepath] [edit filepath]

Convenient to pipe to a .diff file:

    .mariadb_vert_diff from.txt to.txt > output.diff

# Purpose

Quick comparison between results from different versions of mariadb
