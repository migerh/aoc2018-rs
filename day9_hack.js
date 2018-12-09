
function play(numberOfPlayers, lastMarble) {
  let p = 0;
  let marble = 0;
  let scores = new Array(numberOfPlayers).fill(0);

  let current = {
    value: 0,
    next: undefined,
    prev: undefined
  };

  current.prev = current;
  current.next = current;

  while (marble < lastMarble) {
    p = p % numberOfPlayers + 1;
    marble++;

    if (marble % 23 === 0) {
      current = current.prev.prev.prev.prev.prev.prev.prev;
      let value = current.value;
      current.prev.next = current.next;
      current = current.next;

      let newScore = marble + value;
      scores[p - 1] += newScore;

      continue;
    }

    let newMarble = {
      value: marble,
      prev: current.next,
      next: current.next.next,
    };
    current.next.next.prev = newMarble;
    current.next.next = newMarble;

    current = newMarble;
  }

  return Math.max.apply(null, scores);
}

let games = [[9, 25, 32], [10, 1618, 8317], [13, 7999, 146373], [17, 1104, 2764], [21, 6111, 54718], [30, 5807, 37305], [405, 71700, 428690], [405, 7170000, 3628143500]];
games.forEach(([numberOfPlayers, lastScore, expectedHighscore]) => {
  let highScore = play(numberOfPlayers, lastScore);
  console.log(`With ${numberOfPlayers} players and a last score of ${lastScore}, the highscore is ${highScore}`);
  if (highScore !== expectedHighscore) {
    console.log(`WARNING! Wrong highscore! Expected ${expectedHighscore} but got ${highScore}`);
  }
})
