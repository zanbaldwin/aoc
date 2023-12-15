<?php declare(strict_types=1);

namespace ZanBaldwin\Aoc\Day03;

final class UnimplementedException extends \BadMethodCallException
{
    public function __construct(?\Throwable $previous = null)
    {
        parent::__construct('Method has not been implemented.', 0, $previous);
    }
}
