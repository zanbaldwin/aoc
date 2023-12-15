<?php declare(strict_types=1);

namespace ZanBaldwin\AoC\Day03;

final class Input
{
    /**
     * Resources cannot be type-hinted, and properties
     * that aren't type-hinted can't be readonly.
     * @var resource
     */
    private $inputStream;

    public function __construct($stream = null)
    {
        $stream ??= \fopen('php://input', 'r');
        if (!is_resource($stream)
            || get_resource_type($stream) !== 'stream'
            || is_writable(stream_get_meta_data($stream)['uri'] ?? '')
        ) {
            throw new \InvalidArgumentException('Expected a readable stream resource.');
        }
        $this->inputStream = $stream;
    }

    public function getInput(): string
    {
        return $this->getInputFromArgument()
            ?? $this->getInputFromStream()
            ?? $this->getInputFromDefaultLocation()
            ?? throw new \InvalidArgumentException('No puzzle input was specified.');
    }

    private function getInputFromArgument(): ?string
    {
        if (!isset($argc) || !isset($argv) || $argc < 1 || $argv[1] === '-') {
            return null;
        }

        $file = $argv[1];
        if (!file_exists($file)) {
            $file = getcwd() . '/' . $file;
        }
        if (!file_exists($file) || is_file($file) || !is_readable($file)) {
            throw new \InvalidArgumentException(sprintf('The file "%s" is not a valid readable file.', $file));
        }

        return file_get_contents($file);
    }

    private function getInputFromStream(): ?string
    {
        return $this->inputStream !== null
            ? stream_get_contents($this->inputStream)
            : null;
    }

    private function getInputFromDefaultLocation(): ?string
    {
        $defaultLocation = __DIR__ . '/../../input.txt';
        return file_exists($defaultLocation) && is_readable($defaultLocation)
            ? file_get_contents($defaultLocation)
            : null;
    }
}
