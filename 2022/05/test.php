<?php declare(strict_types=1);

namespace Day05;

require_once __DIR__ . '/../common.php';
require_once __DIR__ . '/solution.php';

$input = '    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
';
$solution = new Solution($input);

$test = new \Test($solution, 'CMZ', '');
return $test->runTests();
