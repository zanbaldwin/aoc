<?php declare(strict_types=1);

namespace Day04;

require_once __DIR__ . '/../common.php';

class Solution implements \SolutionInterface
{
    private readonly array $assignmentPairs;

    public function __construct(
        private readonly string $input,
    ) {
        $this->assignmentPairs = array_map(fn (string $line): AssignmentPair => new AssignmentPair($line), split_lines($input, true));
    }

    public function day(): int
    {
        return 4;
    }

    public function part1(): int|string
    {
        return count(array_filter($this->assignmentPairs, fn (AssignmentPair $pair): bool => $pair->isFullyContained()));
    }

    public function part2(): int|string
    {
        return count(array_filter($this->assignmentPairs, fn (AssignmentPair $pair): bool => $pair->isOverlapped()));
    }
}

class AssignmentPair
{
    private readonly SectionRange $first;
    private readonly SectionRange $second;

    public function __construct(public readonly string $line) {
        if (!preg_match('/^\s*(?P<firstFrom>0|[1-9]\d*)\s*-\s*(?P<firstTo>0|[1-9]\d*)\s*,\s*(?P<secondFrom>0|[1-9]\d*)\s*-\s*(?P<secondTo>0|[1-9]\d*)\s*$/', $line, $matches)) {
            throw new \InvalidArgumentException(sprintf('Invalid assignment pair declaration "%s".', $line));
        }
        $this->first = new SectionRange(intval($matches['firstFrom']), intval($matches['firstTo']));
        $this->second = new SectionRange(intval($matches['secondFrom']), intval($matches['secondTo']));
    }

    public function isFullyContained(): bool
    {
        return ($this->first->from <= $this->second->from && $this->first->to >= $this->second->to)
            || ($this->first->from >= $this->second->from && $this->first->to <= $this->second->to);
    }

    public function isOverlapped(): bool
    {
        return ($this->first->from >= $this->second->from && $this->first->from <= $this->second->to)
            || ($this->first->to >= $this->second->from && $this->first->to <= $this->second->to)
            || $this->isFullyContained();
    }
}

class SectionRange
{
    public function __construct(
        public readonly int $from,
        public readonly int $to,
    ) {}
}
