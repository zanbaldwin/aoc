<?php declare(strict_types=1);

namespace Day06;

require_once __DIR__ . '/../common.php';
require_once __DIR__ . '/solution.php';

$return = true;

(new \Test(new Solution('aaaaaaaaaaaaaaaaaqwe'), 20, null))->runTests();
(new \Test(new Solution('mjqjpqmgbljsphdztnvjfqwrcgsmlb'), 7, null))->runTests();
(new \Test(new Solution('bvwbjplbgvbhsrlpgdmjqwftvncz'), 5, null))->runTests();
(new \Test(new Solution('nppdvjthqldpwncqszvftbrmjlhg'), 6, null))->runTests();
(new \Test(new Solution('nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg'), 10, null))->runTests();
(new \Test(new Solution('zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw'), 11, null))->runTests();

(new \Test(new Solution('mjqjpqmgbljsphdztnvjfqwrcgsmlb'), null, 19))->runTests();
(new \Test(new Solution('bvwbjplbgvbhsrlpgdmjqwftvncz'), null, 23))->runTests();
(new \Test(new Solution('nppdvjthqldpwncqszvftbrmjlhg'), null, 23))->runTests();
(new \Test(new Solution('nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg'), null, 29))->runTests();
(new \Test(new Solution('zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw'), null, 26))->runTests();
