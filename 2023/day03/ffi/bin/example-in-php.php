<?php declare(strict_types=1);

use ZanBaldwin\AoC\Day03\Input;
use ZanBaldwin\AoC\Day03\Parser\PhpParser;
use ZanBaldwin\AoC\Day03\Presenter;
use ZanBaldwin\AoC\Day03\Solution;

require_once __DIR__ . '/../autoload.php';

$parser = new PhpParser;
$part1 = new Solution\Part1($parser);
$part2 = new Solution\Part2($parser);

$input = new Input;
$output = fopen('php://stdout', 'a');

$presenter = new Presenter($input->getInput(), $part1, $part2);
$presenter->display($output);
