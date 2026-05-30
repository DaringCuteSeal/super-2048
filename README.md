# Super 2048

## About
It's [2048](https://en.wikipedia.org/wiki/2048_(video_game)) except that instead of only merging once, the numbers can merge recursively:

`2 2 2 2`

`4 4`

`8`

[![asciicast](https://asciinema.org/a/LTlNbbeqwCM63w1L.svg)](https://asciinema.org/a/LTlNbbeqwCM63w1L)

## More About
I wanted to make a 2048 clone, but I accidentally found an O(N)[^1] time complexity and O(1) space complexity recursive merging (which turned out to be easier than limited merging, huh).

[^1]: for one array, so O(N^2) for the whole game board.
