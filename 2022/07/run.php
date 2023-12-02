<?php declare(strict_types=1);

namespace Day07;

require_once __DIR__ . '/../common.php';
require_once __DIR__ . '/solution.php';

$input = get_input(__DIR__);
$solution = new Solution($input);

echo 'Exercise 1: ' . $solution->part1() . PHP_EOL;
echo 'Exercise 2: ' . $solution->part2() . PHP_EOL;
