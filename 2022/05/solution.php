<?php declare(strict_types=1);

namespace Day05;

require_once __DIR__ . '/../common.php';

class Solution implements \SolutionInterface
{
    public function __construct(
        private readonly string $input,
    ) {
        [$cargo, $instructions] = split_blocks($input);
    }

    public function day(): int
    {
        return 5;
    }

    public function part1(): int|string
    {
        return 0;
    }

    public function part2(): int|string
    {
        return 0;
    }
}
