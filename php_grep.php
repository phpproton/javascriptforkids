#!/usr/bin/php
<?php

// Rohit: Grep tool in PHP

//ANSI Escape Codes
$RED = "\e[31m";
$RESET = "\e[0m";


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
    if (preg_match("/($pattern)/", $line, $matches)) {
        $coloredLine = str_replace($matches[1], "$RED{$matches[1]}$RESET", $line);
        echo $coloredLine;
    }
}
fclose($handle);
