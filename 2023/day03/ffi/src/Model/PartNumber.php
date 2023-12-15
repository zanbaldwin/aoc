<?php declare(strict_types=1);

namespace ZanBaldwin\AoC\Day03\Model;

/**
 * @phpstan-import-type CoordShape from Coord
 * @phpstan-type PartNumberShape array{id: int, length: int, coord: CoordShape}
 */
final class PartNumber {
    public function __construct(
        public readonly int $id,
        public readonly int $length,
        public readonly Coord $coord,
    ) {}

    /** @param PartNumberShape $part */
    public static function fromArray(array $part): self {
        return new self(
            $part['id'],
            $part['length'],
            Coord::fromArray($part['coord']),
        );
    }

    public function isNeighbouring(Symbol $symbol): bool {
        $topLeft = new Coord(
            $this->coord->x - 1,
            $this->coord->y - 1,
        );
        $bottomRight = new Coord(
            $this->coord->x + $this->length,
            $this->coord->y + 1,
        );
        // Is the symbol within the bounding box?
        return $symbol->coord->isBoundedBy($topLeft, $bottomRight);
    }
}
