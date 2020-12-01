const PLAYERS: usize = 419;
const MARBLES: usize = 7_105_200;

type NodeHandle = usize;

struct Node {
    value: u32,
    prev: usize,
    next: usize,
}

struct Board {
    nodes: Vec<Node>,
}

impl Board {
    fn new() -> Self {
        let n = Node{ value: 0, prev: 0, next: 0 };
        let nodes = vec![n];
        Board { nodes }
    }

    fn get_start(&self) -> NodeHandle {
        0
    }

    fn insert(&mut self, n: NodeHandle, v: u32) -> NodeHandle {
        let len = self.nodes.len();
        let pn = {
            let prev = &mut self.nodes[n];
            let op = prev.next;
            prev.next = len;
            op
        };
        {
            let next = &mut self.nodes[pn];
            next.prev = len;
        }

        let node = Node {
            value: v,
            prev: n,
            next: pn,
        };

        self.nodes.push(node);
        self.nodes.len() - 1
    }

    fn remove(&mut self, n: NodeHandle) -> (u32, NodeHandle) {
        let (nv, nn, np) = {
            let node = &self.nodes[n];
            (node.value, node.next, node.prev)
        };
        {
            let prev = &mut self.nodes[np];
            prev.next = nn;
        }
        {
            let next = &mut self.nodes[nn];
            next.prev = np;
        }

        (nv, np)
    }

    fn next(&self, n: NodeHandle) -> NodeHandle {
        let node = &self.nodes[n];
        node.next
    }

    fn prev(&self, n: NodeHandle) -> NodeHandle {
        let node = &self.nodes[n];
        node.prev
    }
}

fn main() {
    let mut board = Board::new();
    let mut place = board.get_start();
    let mut scores = vec![0; PLAYERS];

    for m in 1..=MARBLES {
        if m % 23 == 0 {
            let player = m % PLAYERS;
            scores[player] += m as u32;
            for _ in 0..6 {
                place = board.prev(place);
            }
            let (v, p) = board.remove(place);
            scores[player] += v;
            place = p;
        } else {
            place = board.next(place);
            place = board.next(place);
            board.insert(place, m as u32);
        }

        if m == 71052 {
            println!("part 1: {}", scores.iter().max().unwrap());
        }
    }

    println!("part 2: {}", scores.iter().max().unwrap());
}
