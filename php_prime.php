#!/usr/bin/php

<?php

$num = (int)readline('Enter number: ' . PHP_EOL);
$status = ' is prime';

if ($num == 1) {
    $status = ' is not prime';
    }

for ($i = 2; $i <= $num/2; $i++) {
    if ($num % $i == 0) {
    $status = ' is not prime';
    break;
    }
    }

echo 'The number ' . $num . $status . PHP_EOL;

?>
