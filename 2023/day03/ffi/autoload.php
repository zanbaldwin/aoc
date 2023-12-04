<?php declare(strict_types=1);

foreach ([
    // `composer install` was run on this project.
     __DIR__ . '/vendor/autoload.php',
     // This project was added as a dependency of another project.
     __DIR__ . '/../../autoload.php',
] as $autoloader) {
    if (file_exists($autoloader) && is_readable($autoloader)) {
        require_once $autoloader;
        break;
    }
}
