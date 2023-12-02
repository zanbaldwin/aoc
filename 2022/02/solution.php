<?php declare(strict_types=1);

require_once __DIR__ . '/../common.php';

enum Move: int {
    case Rock = 1;
    case Paper = 2;
    case Scissors = 3;
}

enum Outcome: int {
    case Win = 6;
    case Draw = 3;
    case Lose = 0;
}

class Round
{
    public function __construct(
        public readonly Move $opponent,
        public readonly Move $player,
    ) {}

    public function calculateOutcome(): Outcome
    {
        if ($this->opponent->value === $this->player->value) {
            return Outcome::Draw;
        }

        return match ($this->opponent) {
            Move::Rock => match ($this->player) {
                Move::Paper => Outcome::Win,
                Move::Scissors => Outcome::Lose,
            },
            Move::Paper => match ($this->player) {
                Move::Rock => Outcome::Lose,
                Move::Scissors => Outcome::Win,
            },
            Move::Scissors => match ($this->player) {
                Move::Rock => Outcome::Win,
                Move::Paper => Outcome::Lose,
            },
        };
    }

    public function calculatePersonalScore(): int
    {
        return $this->player->value + $this->calculateOutcome()->value;
    }
}

$input = get_input();
$roundDeclarations = split_lines($input, true);

$rounds = array_map(function (string $line): Round {
    if (!preg_match('/^(?P<opponent>[ABC]) (?P<player>[XYZ])$/', $line, $matches)) {
        throw new \InvalidArgumentException('Round declaration not valid.');
    }
    // Remove the full match from the beginning of the matches array.
    array_shift($matches);
    $hands = array_map(fn (string $hand): Move => match ($hand) {
        'A', 'X' => Move::Rock,
        'B', 'Y' => Move::Paper,
        'C', 'Z' => Move::Scissors,
        default => throw new \InvalidArgumentException(sprintf('Invalid hand declaration "%s".', $hand)),
    }, $matches);
    return new Round($hands['opponent'], $hands['player']);
}, $roundDeclarations);

$totalScore = array_sum(array_map(fn (Round $round): int => $round->calculatePersonalScore(), $rounds));
echo sprintf('Your total score of all rounds is %d.', $totalScore) . PHP_EOL;

$rounds = array_map(function (string $line): Round {
    if (!preg_match('/^(?P<opponent>[ABC]) (?P<outcome>[XYZ])$/', $line, $matches)) {
        throw new \InvalidArgumentException('Round declaration not valid.');
    }

    $opponentMove = match ($matches['opponent']) {
        'A' => Move::Rock,
        'B' => Move::Paper,
        'C' => Move::Scissors,
        default => throw new \InvalidArgumentException(sprintf('Invalid hand declaration "%s".', $matches['opponent'])),
    };

    $desiredOutcome = match ($matches['outcome']) {
        'X' => Outcome::Lose,
        'Y' => Outcome::Draw,
        'Z' => Outcome::Win,
        default => throw new \InvalidArgumentException(sprintf('Invalid outcome declaration "%s".', $matches['outcome'])),
    };

    $playerMove = match ($desiredOutcome) {
        Outcome::Draw => $opponentMove,
        Outcome::Win => match ($opponentMove) {
            Move::Rock => Move::Paper,
            Move::Paper => Move::Scissors,
            Move::Scissors => Move::Rock,
        },
        Outcome::Lose => match ($opponentMove) {
            Move::Rock => Move::Scissors,
            Move::Paper => Move::Rock,
            Move::Scissors => Move::Paper,
        },
    };

    return new Round($opponentMove, $playerMove);
}, $roundDeclarations);

$totalScore = array_sum(array_map(fn (Round $round): int => $round->calculatePersonalScore(), $rounds));
echo sprintf('Your total score of all rounds (predetermined outcome) is %d.', $totalScore) . PHP_EOL;
