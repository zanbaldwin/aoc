<?php declare(strict_types=1);

namespace ZanBaldwin\AoC\Day03\Solution;

use ZanBaldwin\AoC\Day03\Model;
use ZanBaldwin\AoC\Day03\Parser\ParserInterface;

final class Part1 implements SolutionInterface
{
    public function __construct(
        private readonly ParserInterface $parser,
    ) {}

    public function solve(string $input): string
    {
        $engineMap = $this->parser->parse($input);
        $partsNeighbouringAnySymbol = array_filter(
            $engineMap->parts,
            fn(Model\PartNumber $part): bool => array_reduce(
                $engineMap->symbols,
                fn(bool $carry, Model\Symbol $symbol): bool => $carry || $part->isNeighbouring($symbol),
                false,
            ),
        );
        $idsOfValidPartNumbers = array_map(
            fn(Model\PartNumber $part): int => $part->id,
            $partsNeighbouringAnySymbol,
        );
        $sumOfValidPartNumbersFor = array_sum($idsOfValidPartNumbers);
        return (string) $sumOfValidPartNumbersFor;
    }
}
