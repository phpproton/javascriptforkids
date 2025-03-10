#!/usr/bin/php

<?php

$numbers = readline('Enter numbers to filter even numbers (comma separated list): ' . PHP_EOL);
$numbers = explode(',', $numbers);

$filteredeven = array_filter($numbers, function($item) {
    return $item % 2 == 0;
});

echo 'These are the even numbers: ' . implode(',', $filteredeven) . PHP_EOL;

?>
