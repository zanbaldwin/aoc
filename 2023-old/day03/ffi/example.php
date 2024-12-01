<?php declare(strict_types=1);

require_once __DIR__ . '/src/Coord.php';
require_once __DIR__ . '/src/EngineMap.php';
require_once __DIR__ . '/src/PartNumber.php';
require_once __DIR__ . '/src/Symbol.php';

$projectRoot = __DIR__ . '/..';
$workspaceRoot = $projectRoot . '/..';
$compilationTargetDirectory = $workspaceRoot . '/target/release';

$ffi = \FFI::cdef(
    file_get_contents($projectRoot . '/ffi/day03.h'),
    // Assuming we're on Linux, use the `.so` (shared object) extension.
    $compilationTargetDirectory . '/libday03.so',
);

// Fetch Puzzle Input
$inputFile = $projectRoot . '/input.txt';
(file_exists($inputFile) && is_readable($inputFile)) ?: throw new \Exception('Input is not a readable file.');
$puzzleInput = file_get_contents($inputFile);
// Parse Puzzle Input using Rust/FFI
$engineMapShape = json_decode($ffi->parse_engine_to_json($puzzleInput), true, flags: \JSON_THROW_ON_ERROR);
$engineMap = EngineMap::fromArray($engineMapShape);

/**
 * Day03 Part 1
 */
$validPartNumbers = $engineMap->getPartsNeighbouringAnySymbol();
$idsOfValidPartNumbers = array_map(
    fn(PartNumber $part): int => $part->id,
    $validPartNumbers,
);
$sumOfValidPartNumbersForPart1 = array_sum($idsOfValidPartNumbers);

/**
 * Day03 Part 2
 */
$gears = $engineMap->getValidGears();
$gearRatios = array_map(
    fn(Gear $gear): int => array_reduce(
        $gear->parts,
        fn(int $carry, PartNumber $part): int => $carry * $part->id,
        1,
    ),
    $gears,
);
$sumOfGearRatiosForPart2 = array_sum($gearRatios);

// Print Results
echo sprintf('Part 1: %d', $sumOfValidPartNumbersForPart1) . PHP_EOL;
echo sprintf('Part 2: %d', $sumOfGearRatiosForPart2) . PHP_EOL;
