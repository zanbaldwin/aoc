<?php declare(strict_types=1);

namespace ZanBaldwin\AoC\Day03\Parser;

use ZanBaldwin\AoC\Day03\Model\EngineMap;
use ZanBaldwin\Aoc\Day03\UnimplementedException;

final class PhpParser implements ParserInterface
{
    /** @throws \InvalidArgumentException */
    public function parse(string $puzzleInput): EngineMap
    {
        throw new UnimplementedException();
    }
}
