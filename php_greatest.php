#!/usr/bin/php

<?php

$num1 = (int)readline('Enter number 1: ' . PHP_EOL);
$num2 = (int)readline('Enter number 2: ' . PHP_EOL);
$num3 = (int)readline('Enter number 3: ' . PHP_EOL);

$numbers = [$num1, $num2, $num3];
sort($numbers);

echo 'Greatest number is: ' . end($numbers) . '.' . PHP_EOL;

?>
