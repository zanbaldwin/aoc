<?php declare(strict_types=1);

$ffi = \FFI::cdef(
    file_get_contents(__DIR__ . '/day03.h'),
    // Assuming we're on Linux, use the `.so` (shared object) extension.
    __DIR__ . '/../target/release/libday03.so',
);

$inputFile = __DIR__ . '/input.txt';
(file_exists($inputFile) && is_readable($inputFile)) ?: throw new \Exception('Input is not a readable file.');

echo $ffi->parse_engine_to_json(file_get_contents($inputFile)) . PHP_EOL;
