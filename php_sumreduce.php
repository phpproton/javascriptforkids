#!/usr/bin/php

<?php
 
$numbers = readline('Enter numbers to add (comma separated list): ' . PHP_EOL);
$numbers = explode(',', $numbers);

//$previous is the initial value which is entered in the third parameter as 0 or anything
$total = array_reduce($numbers, function($previous, $current)
{
    return $previous + $current;
}, 0);

echo 'The sum of all the numbers is: ' . $total . PHP_EOL;

?>
