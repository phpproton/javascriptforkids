#!/usr/bin/php

<?php

$num = (int)readline('Enter number: ' . PHP_EOL);

$sum = array_sum(str_split($num));

echo 'Sum of digits is: ' . $sum . PHP_EOL;

?>
