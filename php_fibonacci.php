#!/usr/bin/php

<?php

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

?>
