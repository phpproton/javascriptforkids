#!/usr/bin/php

<?php

$num = (int)readline('Enter number: ' . PHP_EOL);
$status = null;

if (strrev($num) == $num){ 
    $status = 'is'; 
    } else {
    $status = 'is not';
    }

echo 'The number ' . $status . ' a palindrome ' . PHP_EOL;

?>
