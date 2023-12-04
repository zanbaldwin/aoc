<?php declare(strict_types=1);

readonly class EngineMap
{
    /**
     * @param array<PartNumber> $parts
     * @param array<Symbol> $symbols
     */
    public function __construct(
        public array $parts,
        public array $symbols,
    ) {}

    /** @param array{parts: array<PartNumberShape>, symbols: array<SymbolShape>} $engineMap */
    public static function fromArray(array $engineMap): self {
        return new self(
            array_map([PartNumber::class, 'fromArray'], $engineMap['parts']),
            array_map([Symbol::class, 'fromArray'], $engineMap['symbols']),
        );
    }

    /** @return arraay<PartNumber> */
    public function getPartsNeighbouringAnySymbol(): array {
        return array_filter(
            $this->parts,
            fn(PartNumber $part): bool => array_reduce(
                $this->symbols,
                fn(bool $carry, Symbol $symbol): bool => $carry || $part->isNeighbour($symbol),
                false,
            ),
        );
    }

    /** @return array<Gear> */
    public function getValidGears(): array {
        return array_filter(
            $this->getGears(),
            fn(Gear $gear): bool => count($gear->parts) === 2,
        );
    }

    /** @return array<Gear> */
    public function getGears(): array {
        $symbolsThatAreGears = array_filter(
            $this->symbols,
            fn(Symbol $symbol): bool => $symbol->symbol === '*',
        );
        return array_map(fn(Symbol $symbol): Gear => new Gear(
            $symbol,
            $this->getAdjacentParts($symbol),
        ), $symbolsThatAreGears);
    }

    /** @return array<PartNumber> */
    private function getAdjacentParts(Symbol $symbol): array {
        return array_filter(
            $this->parts,
            fn(PartNumber $part): bool => $part->isNeighbour($symbol),
        );
    }
}

readonly class Gear {
    public function __construct(
        public Symbol $symbol,
        public array $parts,
    ) {}
}
