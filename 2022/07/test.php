<?php declare(strict_types=1);

namespace Day07;

require_once __DIR__ . '/../common.php';
require_once __DIR__ . '/solution.php';

$input = '';
$solution = new Solution($input);

return (new \Test($solution, 7, null))->runTests();
