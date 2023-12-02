<?php declare(strict_types=1);

interface SolutionInterface
{
    public function __construct(string $input);
    public function day(): int;
    public function part1(): int|string;
    public function part2(): int|string;
}

class Test
{
    public const SUCCESS = 0;
    public const FAIL = 1;

    private readonly array $answer;

    public function __construct(
        private readonly SolutionInterface $solution,
        int|string|null $testAnswer1,
        int|string|null $testAnswer2,
    ) {
        $this->answer = [
            1 => $testAnswer1,
            2 => $testAnswer2,
        ];
    }

    public function runTests(): bool
    {
        $output = [
            sprintf('# Running tests for Day %02d', $this->solution->day()),
        ];

        $answer = [
            1 => $this->answer[1] !== null ? $this->solution->part1() : null,
            2 => $this->answer[2] !== null ? $this->solution->part2() : null,
        ];
        $result = [
            1 => $answer[1] === $this->answer[1],
            2 => $answer[2] === $this->answer[2],
        ];

        foreach ([1, 2] as $part) {
            if ($this->answer[$part] === null) {
                continue;
            }
            $output[] = $result[$part]
                ? sprintf('[SUCCESS] The answer to part %d "%s" is correct.', $part, $answer[$part])
                : sprintf('[   FAIL] The answer to part %d "%s" does not match "%s".', $part, $answer[$part], $this->answer[$part]);
        }

        echo implode(PHP_EOL, $output) . PHP_EOL . PHP_EOL;
        return $result[1] && $result[2];
    }
}

function get_input(?string $workingDirectory = null): string
{
    global $argv;
    if (!is_string($filename = $argv[1] ?? null)) {
        $filename = ($workingDirectory ?? __DIR__) . '/input.txt';
    }
    if (!file_exists($filename) || !is_readable($filename)) {
        throw new \InvalidArgumentException(sprintf('File "%s" is not a valid, readable file.', $filename));
    }
    if (false === $contents = file_get_contents($filename)) {
        throw new \InvalidArgumentException(sprintf('Could not read contents of file "%s".', $filename));
    }
    return $contents;
}

function split_blocks(string $input, bool $ignoreEmpty = false): array
{
    $separator = str_contains($input, "\r\n") ? "\r\n" : "\n";
    $blocks = explode(str_repeat($separator, 2), $input);
    if ($ignoreEmpty) {
        $blocks = array_filter($blocks, fn (string $block): bool => $block !== '');
    }
    return $blocks;
}

function split_lines(string $input, bool $ignoreEmpty = false): array
{
    $separator = str_contains($input, "\r\n") ? "\r\n" : "\n";
    $lines = explode($separator, $input);
    if ($ignoreEmpty) {
        $lines = array_filter($lines, fn (string $line): bool => $line !== '');
    }
    return $lines;
}
