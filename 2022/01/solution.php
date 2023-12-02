<?php declare(strict_types=1);

require_once __DIR__ . '/../common.php';

$input = get_input();
$separator = str_contains($input, "\r\n") ? "\r\n" : "\n";

$elves = array_map(
    fn (string $calorieDeclarationBlock): int => array_reduce(
        array_map(
            fn (string $calorieDeclaration): int => intval($calorieDeclaration),
            split_lines($calorieDeclarationBlock),
        ),
        fn (int $carry, int $item): int => $carry + $item,
        0,
    ),
    split_blocks($input),
);

arsort($elves, \SORT_NUMERIC);
$elfWithMax = array_key_first($elves);

echo sprintf('Elf #%d has the most calories with a total of %d.', $elfWithMax + 1, $elves[$elfWithMax]) . PHP_EOL;

$topThreeElves = array_slice($elves, 0, 3, true);
echo sprintf(
    'The top three elves (%s) are carrying a combined total of %d calories.',
    implode(', ', array_map(fn (int $key): int => $key + 1, array_keys($topThreeElves))),
    array_sum($topThreeElves),
) . PHP_EOL;
