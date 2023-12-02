<?php declare(strict_types=1);

namespace Day06;

require_once __DIR__ . '/../common.php';

class Solution implements \SolutionInterface
{
    /** @var string[] */
    private readonly array $stream;

    public function __construct(
        private readonly string $input,
    ) {
        $this->stream = str_split($input);
    }

    public function day(): int
    {
        return 6;
    }

    public function part1(): int|string
    {
        $packetMarkerLength = 4;

        $previous = array_slice($this->stream, 0, $packetMarkerLength - 1);
        $stream = array_slice($this->stream, $packetMarkerLength - 1);

        $length = count($stream) + $packetMarkerLength;
        for ($i = $packetMarkerLength; $i < $length; $i++) {
            $byte = array_shift($stream);
            $previous[] = $byte;
            if (!self::hasDuplicates($previous)) {
                return $i;
            }
            array_shift($previous);
        }
        throw new \InvalidArgumentException('Packet marker not found.');
    }

    public function part2(): int|string
    {
        $messageMarkerLength = 14;

        $previous = array_slice($this->stream, 0, $messageMarkerLength - 1);
        $stream = array_slice($this->stream, $messageMarkerLength - 1);

        $length = count($stream) + $messageMarkerLength;
        for ($i = $messageMarkerLength; $i < $length; $i++) {
            $byte = array_shift($stream);
            $previous[] = $byte;
            if (!self::hasDuplicates($previous)) {
                return $i;
            }
            array_shift($previous);
        }
        throw new \InvalidArgumentException('Message marker not found.');
    }

    public static function hasDuplicates(array $arr): bool
    {
        return count($arr) !== count(array_flip($arr));
    }
}
