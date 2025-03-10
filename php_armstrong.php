#!/usr/bin/php

<?php

$num = (int)readline('Enter number: ' . PHP_EOL);
$status = ' is not Armstrong.';

$numarray = str_split($num);
$power = count($numarray);

$sum = array_sum(array_map(function($item) use ($power) {
    return pow($item, $power);
}
, $numarray));

if ($num == $sum ) {
    $status = ' is Armstrong.';
    }

echo 'The number ' . $num . $status . PHP_EOL;

?>
