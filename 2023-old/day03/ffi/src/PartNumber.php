<?php declare(strict_types=1);

/**
 * @phpstan-import-type CoordShape from Coord
 * @phpstan-type PartNumberShape array{id: int, length: int, coord: CoordShape}
 */
readonly class PartNumber {
    public function __construct(
        public int $id,
        public int $length,
        public Coord $coord,
    ) {}

    /** @param PartNumberShape $part */
    public static function fromArray(array $part): self {
        return new self(
            $part['id'],
            $part['length'],
            Coord::fromArray($part['coord']),
        );
    }

    public function isNeighbour(Symbol $symbol): bool {
        $topleft = new Coord(
            max(0, $this->coord->x - 1),
            max(0, $this->coord->y - 1),
        );
        $bottomright = new Coord(
            $this->coord->x + $this->length,
            $this->coord->y + 1,
        );
        // Is the symbol within the bounding box?
        return $symbol->coord->isBoundedBy($topleft, $bottomright);
    }
}
