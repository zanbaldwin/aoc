<?php declare(strict_types=1);

namespace Day03;

require_once __DIR__ . '/../common.php';
require_once __DIR__ . '/solution.php';

$input = 'vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw';
$solution = new Solution($input);

echo sprintf("Exercise 1: %s [%s]\n", $solution->part1(), $solution->part1() === 157 ? 'SUCCESS' : 'FAIL');
echo sprintf("Exercise 1: %s [%s]\n", $solution->part2(), $solution->part2() === 70 ? 'SUCCESS' : 'FAIL');
