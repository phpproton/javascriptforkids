#!/usr/bin/php
<?php

// Rohit: Grep tool in PHP

if ($argc < 3) {
    fwrite(STDERR, "Usage: {$argv[0]} <pattern> <file>\n");
    exit(1);
}

$pattern = $argv[1];
$file = $argv[2];

$handle = fopen($file, 'r');

if (!$handle) {
    fwrite(STDERR, "Error: Cannot open $file\n");
    exit(1);
}

while (($line = fgets($handle)) !== false) {
    if (preg_match("/$pattern/", $line))
        echo $line;
}

fclose($handle);
