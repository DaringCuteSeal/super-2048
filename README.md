# Super 2048

## About
It's [2048](https://en.wikipedia.org/wiki/2048_(video_game)) except that instead of only merging once, the numbers can merge recursively:

`2 2 2 2`

`4 4`

`8`

<a href="https://asciinema.org/a/144xnRtply1uUJau" target="_blank"><img src="https://asciinema.org/a/144xnRtply1uUJau.svg" /></a>

## More About
I wanted to make a 2048 clone, but I accidentally found an O(N)[^1] time complexity and O(1) space complexity recursive merging (which turned out to be easier than limited merging, huh).

## FAQ

<details>
  <summary>How do I actually control it?</summary>
You must not be a vim user, huh? Use h for left, j for down, k for up, and l for right.
</details>

<details>
  <summary>I'm too good at this game but I'm too tired to continue, how do I quit?</summary>
Press q.
</details>

[^1]: for one array, so O(N^2) for the whole game board.
