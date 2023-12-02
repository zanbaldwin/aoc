<?php declare(strict_types=1);

namespace Day03;

require_once __DIR__ . '/../common.php';

class Solution implements \SolutionInterface
{
    /** @var Rucksack[] */
    private array $rucksacks;

    public function __construct(
        private readonly string $input,
    ) {
        $this->rucksacks = array_map(fn (string $line): Rucksack => new Rucksack($line), split_lines($input, true));
    }

    public function day(): int
    {
        return 3;
    }

    public function part1(): int|string
    {
        $itemPriorities = array_map(fn (Rucksack $rucksack): int => Item::getPriority($rucksack->getMatchingItem()), $this->rucksacks);
        return array_sum($itemPriorities);
    }

    public function part2(): int|string
    {
        $groups = array_chunk($this->rucksacks, 3, false);
        $groupBadgePriorities = array_map(function (array $rucksacks): int {
            $groupBadge = $rucksacks[0]->findGroupBadge($rucksacks[1], $rucksacks[2]);
            $groupBadgePriority = Item::getPriority($groupBadge);
            return $groupBadgePriority;
        }, $groups);

        return array_sum($groupBadgePriorities);
    }
}

class Rucksack
{
    public readonly string $firstCompartment;
    public readonly string $secondCompartment;

    public function __construct(public readonly string $line)
    {
        $lineLength = strlen($line);
        if ($lineLength % 2 === 1) {
            throw new \InvalidArgumentException(sprintf('Rucksack declaration contains %d items.', $lineLength));
        }
        $this->firstCompartment = substr($line, 0, $lineLength / 2);
        $this->secondCompartment = substr($line, $lineLength / 2);
    }

    public function getMatchingItem(): string
    {
        $matchingItems = [];
        foreach (str_split($this->firstCompartment) as $item) {
            if (str_contains($this->secondCompartment, $item)) {
                $matchingItems[$item] = true;
            }
        }
        if (count($matchingItems) !== 1) {
            throw new \RuntimeException(sprintf('%d matching items were found in compartments.', count($matchingItems)));
        }
        return array_key_first($matchingItems);
    }

    public function findGroupBadge(Rucksack $second, Rucksack $third): string
    {
        $matchingItems = [];
        foreach(str_split($this->line) as $item) {
            if (str_contains($second->line, $item) && str_contains($third->line, $item)) {
                $matchingItems[$item] = true;
            }
        }
        if (count($matchingItems) !== 1) {
            throw new \RuntimeException(sprintf('%d matching items were found in rucksacks.', count($matchingItems)));
        }
        return array_key_first($matchingItems);
    }
}

class Item
{
    public static function getPriority(string $item): int
    {
        if (strlen($item) !== 1) {
            throw new \InvalidArgumentException(sprintf('Item must be a single character, "%s" given.', $item));
        }
        $ord = ord($item);
        if ($ord >= 97 && $ord <= 122) {
            return $ord - 96;
        } elseif ($ord >= 65 && $ord <= 90) {
            return $ord - 38;
        }
        throw new \InvalidArgumentException(sprintf('Item must be lower or uppercase Latin alphabet letter, "%s" given.', $item));
    }
}
