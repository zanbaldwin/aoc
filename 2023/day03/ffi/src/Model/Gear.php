<?php declare(strict_types=1);

namespace ZanBaldwin\AoC\Day03\Model;

final class Gear {
    /** @param PartNumber[] $parts */
    public function __construct(
        public readonly Symbol $symbol,
        public readonly array $parts,
    ) {}
}
