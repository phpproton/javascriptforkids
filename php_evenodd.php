#!/usr/bin/php

<?php

$num = (int)readline('Enter number: ' . PHP_EOL);
$status = null;

if($num % 2 == 0) {
    $status = 'even.';
    } else {
    $status = 'odd.';
    }

echo 'The number ' . $num . ' is ' . $status . PHP_EOL;

?>
