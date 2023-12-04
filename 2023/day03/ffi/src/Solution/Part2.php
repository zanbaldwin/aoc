<?php declare(strict_types=1);

namespace ZanBaldwin\AoC\Day03\Solution;

use ZanBaldwin\AoC\Day03\Model;
use ZanBaldwin\AoC\Day03\Parser\ParserInterface;

final class Part2 implements SolutionInterface
{
    public const GEAR_CHARACTER = '*';

    public function __construct(
        private readonly ParserInterface $parser,
    ) {}

    public function solve(string $input): string
    {
        $engineMap = $this->parser->parse($input);
        $gears = $this->getGears($engineMap);
        $ratioCalculator = fn(Model\Gear $gear): int => array_reduce(
            $gear->parts,
            fn(int $carry, Model\PartNumber $part): int => $carry * $part->id,
            1,
        );
        $gearRatios = array_map($ratioCalculator, $gears);
        $sumOfGearRatios = array_sum($gearRatios);
        return (string) $sumOfGearRatios;
    }

    /** @return Model\Gear[] */
    private function getGears(Model\EngineMap $engineMap): array
    {
        $symbolsWithCorrectCharacter = array_filter(
            $engineMap->symbols,
            fn(Model\Symbol $symbol): bool => $symbol->symbol === self::GEAR_CHARACTER,
        );
        $gears = array_map(fn(Model\Symbol $gearSymbol): Model\Gear => new Model\Gear(
            $gearSymbol,
            $this->getPartsAdjacentToGearSymbol($engineMap, $gearSymbol),
        ), $symbolsWithCorrectCharacter);
        $validGears = array_filter(
            $gears,
            fn(Model\Gear $gear): bool => count($gear->parts) === 2,
        );
        return $validGears;
    }

    private function getPartsAdjacentToGearSymbol(
        Model\EngineMap $engineMap,
        Model\Symbol $gearSymbol,
    ): array {
        return array_filter(
            $engineMap->parts,
            fn(Model\PartNumber $part): bool => $part->isNeighbouring($gearSymbol),
        );
    }
}
