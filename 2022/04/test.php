<?php declare(strict_types=1);

namespace Day04;

require_once __DIR__ . '/../common.php';
require_once __DIR__ . '/solution.php';

$input = '2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8';
$solution = new Solution($input);

return (new \Test($solution, 2, 4))->runTests()
    ? \Test::SUCCESS
    : \Test::FAIL;
