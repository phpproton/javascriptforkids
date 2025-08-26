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

$num = (int)readline('Enter number: ' . PHP_EOL);
$status = null;

if($num % 2 == 0) {
    $status = 'even.';
    } else {
    $status = 'odd.';
    }

echo 'The number ' . $num . ' is ' . $status . PHP_EOL;

$num = (int)readline('Enter factorial number: ' . PHP_EOL);

function factorial($num) {
    if($num <= 1) {
    return 1;
    } else {
    return $num * factorial($num-1);
    }
}

echo 'Factorial of ' . $num . ' is ' . factorial($num) . '.' . PHP_EOL;

$n1 = 0;
$n2 = 1;
$n3 = 1;

$limit = (int)readline('Enter Fibonacci limit: ' . PHP_EOL);

echo $n1 . ',' . $n2;

while($n3 < $limit) {
    echo ',' . $n3;
    $n1 = $n2;
    $n2 = $n3;
    $n3 = $n1 + $n2;
}

echo PHP_EOL;

$numbers = readline('Enter numbers to filter even numbers (comma separated list): ' . PHP_EOL);
$numbers = explode(',', $numbers);

$filteredeven = array_filter($numbers, function($item) {
    return $item % 2 == 0;
});

echo 'These are the even numbers: ' . implode(',', $filteredeven) . PHP_EOL;

$num1 = (int)readline('Enter number 1: ' . PHP_EOL);
$num2 = (int)readline('Enter number 2: ' . PHP_EOL);
$num3 = (int)readline('Enter number 3: ' . PHP_EOL);

$numbers = [$num1, $num2, $num3];
sort($numbers);

echo 'Greatest number is: ' . end($numbers) . '.' . PHP_EOL;

$guess = null;
$tries = 0;
$number = mt_rand(1, 10);

while($guess != $number) {
    $guess = (int)readline('Guess a number: ' . PHP_EOL);
    $tries++;
}

echo 'You guessed the number ' . $number . ' in ' . $tries . ' tries.' . PHP_EOL;

$num = (int)readline('Enter number: ' . PHP_EOL);
$status = null;

if (strrev($num) == $num){ 
    $status = 'is'; 
    } else {
    $status = 'is not';
    }

echo 'The number ' . $status . ' a palindrome ' . PHP_EOL;

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

$num = (int)readline('How big your star is (Enter 10-40) or even more): ' . PHP_EOL);

for($i=1; $i<=$num; $i++) {
    echo str_repeat(' ', $num - $i) . str_repeat('*', $i) . str_repeat('*', $i) . str_repeat(' ', $num - $i) . PHP_EOL;
}

for($i=$num; $i>=1; $i--) {
    echo str_repeat(' ', $num - $i) . str_repeat('*', $i) . str_repeat('*', $i) . str_repeat(' ', $num - $i) . PHP_EOL;
}

$num = (int)readline('Enter number: ' . PHP_EOL);

$sum = array_sum(str_split($num));

echo 'Sum of digits is: ' . $sum . PHP_EOL;

$numbers = readline('Enter numbers to add (comma separated list): ' . PHP_EOL);
$numbers = explode(',', $numbers);

//$previous is the initial value which is entered in the third parameter as 0 or anything
$total = array_reduce($numbers, function($previous, $current)
{
    return $previous + $current;
}, 0);

echo 'The sum of all the numbers is: ' . $total . PHP_EOL;

$num = (int)readline('Enter number: ' . PHP_EOL);

echo 'Table of ' . $num . ':' . PHP_EOL;

for($i = 1; $i < 11; $i++) 
    {
    echo $num . ' X ' . $i . ' = ' . $num * $i . PHP_EOL;
    }

// Rohit: Grep tool in PHP
//ANSI Escape Codes
$RED = "\e[31m";
$RESET = "\e[0m";

if ($argc < 3) {
    fwrite(STDERR, "Usage: {$argv[0]} <pattern> <file>\n");
    exit(1);
}

$pattern = $argv[1];
$file = $argv[2];

$handle = fopen($file, 'r');

if (!$handle) {
    fwrite(STDERR, "Error: Cannot open $file\n");
    exit(1);
}

while (($line = fgets($handle)) !== false) {
    if (preg_match("/$pattern/", $line, $matches)) {
        $coloredline = str_replace($matches[0], "$RED{$matches[0]}$RESET", $line);
        //print_r($matches);
        echo $coloredline;
    }
}
fclose($handle);

?>
