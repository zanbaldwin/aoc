<?php declare(strict_types=1);

namespace ZanBaldwin\AoC\Day03\Model;

/**
 * @phpstan-import-type PartNumberShape from PartNumber
 * @phpstan-import-type SymbolShape from Symbol
 */
final class EngineMap
{
    /**
     * @param array<PartNumber> $parts
     * @param array<Symbol> $symbols
     */
    public function __construct(
        public readonly array $parts,
        public readonly array $symbols,
    ) {}

    /** @param array{parts: PartNumberShape[], symbols: SymbolShape[]} $engineMap */
    public static function fromArray(array $engineMap): self {
        return new self(
            array_map([PartNumber::class, 'fromArray'], $engineMap['parts']),
            array_map([Symbol::class, 'fromArray'], $engineMap['symbols']),
        );
    }
}
