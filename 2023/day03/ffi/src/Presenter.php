<?php declare(strict_types=1);

namespace ZanBaldwin\AoC\Day03;

use ZanBaldwin\AoC\Day03\Solution\SolutionInterface;

final class Presenter implements \Stringable
{
    public function __construct(
        private readonly string $puzzleInput,
        private readonly SolutionInterface $part1,
        private readonly SolutionInterface $part2,
    ) {}

    public function render(): string
    {
        return implode(\PHP_EOL, [
            sprintf('Answer to Part 1: %s', $this->part1->solve($this->puzzleInput)),
            sprintf('Answer to Part 2: %s', $this->part2->solve($this->puzzleInput))
        ]) . \PHP_EOL;
    }

    public function display($stream): void
    {
        if (!is_resource($stream)
            || get_resource_type($stream) !== 'stream'
            || is_writable(stream_get_meta_data($stream)['uri'] ?? '')
        ) {
            throw new \InvalidArgumentException('Expected a writable stream resource.');
        }
        \fwrite($stream, $this->render(), null);
    }

    public function __toString(): string
    {
        return $this->render();
    }
}
