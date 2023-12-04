<?php declare(strict_types=1);

namespace ZanBaldwin\AoC\Day03\Model;

/**
 * @phpstan-import-type CoordShape from Coord
 * @phpstan-type SymbolShape array{symbol: string, coord: CoordShape}
 */
final class Symbol {
    public function __construct(
        public readonly string $symbol,
        public readonly Coord $coord,
    ) {}

    /** @param SymbolShape $symbol */
    public static function fromArray(array $symbol): self {
        return new self($symbol['symbol'], Coord::fromArray($symbol['coord']));
    }
}
