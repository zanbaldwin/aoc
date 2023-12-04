<?php declare(strict_types=1);

namespace ZanBaldwin\AoC\Day03\Parser;

use ZanBaldwin\AoC\Day03\Model\EngineMap;

interface ParserInterface
{
    public function parse(string $puzzleInput): EngineMap;
}
