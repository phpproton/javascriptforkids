#!/usr/bin/php

<?php

$guess = null;
$tries = 0;
$number = mt_rand(1, 10);

while($guess != $number) {
    $guess = (int)readline('Guess a number: ' . PHP_EOL);
    $tries++;
}

echo 'You guessed the number ' . $number . ' in ' . $tries . ' tries.' . PHP_EOL;

?>
