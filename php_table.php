#!/usr/bin/php

<?php

$num = (int)readline('Enter number: ' . PHP_EOL);

echo 'Table of ' . $num . ':' . PHP_EOL;

for($i = 1; $i < 11; $i++) 
    {
    echo $num . ' X ' . $i . ' = ' . $num * $i . PHP_EOL;
    }

?>
