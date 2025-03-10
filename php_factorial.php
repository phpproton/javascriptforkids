#!/usr/bin/php

<?php

$num = (int)readline('Enter factorial number: ' . PHP_EOL);

function factorial($num) {
    if($num <= 1) {
    return 1;
    } else {
    return $num * factorial($num-1);
    }
}

echo 'Factorial of ' . $num . ' is ' . factorial($num) . '.' . PHP_EOL;

?>
