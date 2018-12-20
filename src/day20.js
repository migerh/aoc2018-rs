const fs = require('fs');
const raw = fs.readFileSync('./src/day20/data/input.txt', 'utf8');
// const raw = '^ENWWW(NEEE|SSE(EE|N))$';
// const raw = '^ENNWSWW(NEWS|)SSSEEN(WNSE|)EE(SWEN|)NNN$';
// const raw = '^ESSWWN(E|NNENN(EESS(WNSE|)SSS|WWWSSSSE(SW|NNNE)))$';
const relevant = raw.slice(1, -1);
console.log('length:', relevant.length);

const dirs = new Set(['E', 'N', 'W', 'S']);
function isDirection(char) {
  return dirs.has(char);
}

function freshNode() {
  return {
    buffer: '',
    children: []
  }
}

function parse(s) {
  // let node = {
  //   buffer: '',
  //   children: []
  // };
  let nodes = [];
  let node = freshNode();
  while(s.length > 0) {
    let char = s.splice(0, 1).join('');
    if (isDirection(char)) {
      node.buffer += char;
    } else if (char ===  '(') {
      node.children = parse(s);
    } else if (char === '|') {
      nodes.push(node);
      node = freshNode();
    } else if (char === ')') {
      nodes.push(node);
      return nodes;
    }
  }
  return node;
}

let nodes = parse(relevant.split(''));

let atleast1000 = 0;
function shortestPath(root, baseLength = 0) {
  const length = baseLength + root.buffer.length;

  if (length >= 1000) {
    if (baseLength >= 1000) {
      atleast1000 += root.buffer.length;
    } else {
      let notyet1000 = 1000 - baseLength;
      atleast1000 += root.buffer.length - notyet1000;
    }
  }

  if (root.children.length === 0) {
    return length;
  }

  let lengths = [];
  for (child of root.children) {
    lengths.push(shortestPath(child, length));
  }
  return Math.max.apply(Math, lengths);
}

let length = shortestPath(nodes);

console.log(JSON.stringify(nodes, null, 2));
console.log('length', length);
console.log('at least 1000', atleast1000);
