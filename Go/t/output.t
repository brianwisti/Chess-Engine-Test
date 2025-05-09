#!/usr/bin/env perl

use 5.40.0;
use warnings;

use Test::More tests => 1;

my $EXPECTED_OUTPUT =<<END;
b1a3: 4856835
b1c3: 5708064
g1f3: 5723523
g1h3: 4877234
a2a3: 4463267
a2a4: 5363555
b2b3: 5310358
b2b4: 5293555
c2c3: 5417640
c2c4: 5866666
d2d3: 8073082
d2d4: 8879566
e2e3: 9726018
e2e4: 9771632
f2f3: 4404141
f2f4: 4890429
g2g3: 5346260
g2g4: 5239875
h2h3: 4463070
h2h4: 5385554
END
;

my $raw_output = `go run .`;
my ($board, $moves, $summary) = split "-----\n", $raw_output;

is $moves, $EXPECTED_OUTPUT, "move listing matches";
