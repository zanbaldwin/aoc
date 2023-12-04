<?php declare(strict_types=1);

namespace ZanBaldwin\AoC\Day03\Model;

/** @phpstan-type CoordShape array{x: int, y: int} */
final class Coord {
    public function __construct(
        public readonly int $x,
        public readonly int $y,
    ) {}

    /** @param CoordShape $coord */
    public static function fromArray(array $coord): self {
        return new self($coord['x'], $coord['y']);
    }

    public function isBoundedBy(self $corner1, self $corner2): bool {
        return $this->x >= min($corner1->x, $corner2->x)
            && $this->x <= max($corner1->x, $corner2->x)
            && $this->y >= min($corner1->y, $corner2->y)
            && $this->y <= max($corner1->y, $corner2->y);
    }
}
