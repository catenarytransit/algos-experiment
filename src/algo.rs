use pathfinding::prelude::dijkstra;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos(i32, i32);

impl Pos {
  fn successors(&self) -> Vec<(Pos, usize)> {
    let &Pos(x, y) = self;
    vec![Pos(x+1,y+2), Pos(x+1,y-2), Pos(x-1,y+2), Pos(x-1,y-2),
         Pos(x+2,y+1), Pos(x+2,y-1), Pos(x-2,y+1), Pos(x-2,y-1)]
         .into_iter().map(|p| (p, 1)).collect()
  }
}

fn main() {
    let result = dijkstra(&Pos(1, 1), |p| p.successors(), |p| *p == GOAL);
    static GOAL: Pos = Pos(4, 6);
    assert_eq!(result.clone().expect("no path found").1, 4);
    println!("{:?}", result);
}
