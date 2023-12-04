<?php declare(strict_types=1);

namespace ZanBaldwin\AoC\Day03\Parser;

use ZanBaldwin\AoC\Day03\Model\EngineMap;

final class FfiParser implements ParserInterface
{
    public const DEFAULT_PROJECT_NAME = 'day03';
    public const DEFAULT_LIBRARY_NAME = 'lib' . self::DEFAULT_PROJECT_NAME;
    public const DEFAULT_RUST_BUILD_PROFILE = 'release';

    private readonly \FFI $ffi;

    /** @throws \FFI\Exception */
    public function __construct(
        public readonly string $libraryName = self::DEFAULT_LIBRARY_NAME,
        public readonly string $rustBuildProfile = self::DEFAULT_RUST_BUILD_PROFILE,
    ) {
        $phpProjectRoot = __DIR__ . '/../..';
        $rustProjectRoot = $phpProjectRoot . '/..';
        $this->ffi = \FFI::cdef(
            sprintf('%s/%s.h', $rustProjectRoot, $this->libraryName),
            self::guessLibraryLocation($this->rustBuildProfile, $this->libraryName),
        );
    }

    private static function guessLibraryLocation(string $buildProfile, string $libraryName): string
    {
        $possibleLocations = [
            // Rust project is part of a workspace which shares a target directory.
            __DIR__ . '/../../../../target',
            // Rust project was built independently.
            __DIR__ . '/../../../target',
        ];

        $checked = [];
        foreach ($possibleLocations as $possibleTargetDir) {
            $checked[] = $libraryPath = sprintf(
                '%s/%s/%s.%s',
                $possibleTargetDir,
                $buildProfile,
                $libraryName,
                self::platformExtension(),
            );
            if (file_exists($libraryPath)) {
                return $libraryPath;
            }
        }

        throw new \RuntimeException('Could not locate Rust-based shared library; checked for: ' . implode(
            '; ',
            $checked,
        ));
    }

    private static function platformExtension(): string
    {
        return match (\PHP_OS_FAMILY) {
            'Windows' => 'dll',
            'OSC' => 'dylib',
            default => 'so',
        };
    }

    /**
     * @throws \JsonException
     * @throws \FFI\Exception
     */
    public function parse(string $puzzleInput): EngineMap
    {
        $engineJson = $this->ffi->parse_engine_textmap_to_json($puzzleInput);
        $engineShape = json_decode($engineJson, true, flags: \JSON_THROW_ON_ERROR);
        return EngineMap::fromArray($engineShape);
    }
}
