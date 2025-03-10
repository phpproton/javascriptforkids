#!/usr/bin/php

<?php

$num = (int)readline('How big your star is (Enter 10-40) or even more): ' . PHP_EOL);

for($i=1; $i<=$num; $i++) {
    echo str_repeat(' ', $num - $i) . str_repeat('*', $i) . str_repeat('*', $i) . str_repeat(' ', $num - $i) . PHP_EOL;
}

for($i=$num; $i>=1; $i--) {
    echo str_repeat(' ', $num - $i) . str_repeat('*', $i) . str_repeat('*', $i) . str_repeat(' ', $num - $i) . PHP_EOL;
}

?>
