<?php declare(strict_types=1);

/**
 * @phpstan-import-type CoordShape from Coord
 * @phpstan-type SymbolShape array{symbol: string, coord: CoordShape}
 */
readonly class Symbol {
    public function __construct(
        public string $symbol,
        public Coord $coord,
    ) {}

    /** @param SymbolShape $symbol */
    public static function fromArray(array $symbol): self {
        return new self($symbol['symbol'], Coord::fromArray($symbol['coord']));
    }
}
