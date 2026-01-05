struct Solution;

struct UnionFind{
    parent: Vec<usize>,
    rank: Vec<usize>,
}

impl UnionFind {
    pub fn new(n: usize) -> Self {
        let mut parent = vec![0; n];
        for (ii, element) in parent.iter_mut().enumerate() {
            *element = ii;
        }
        let rank = vec![1; n];
        UnionFind{
            parent,
            rank,
        }
    }

    pub fn union(&mut self, mut left: usize, mut right: usize) -> usize {
        left = self.find(left);
        right = self.find(right);
        if self.rank[left] < self.rank[right] {
            self.parent[left] = right;
            self.rank[right] += self.rank[left];
            right
        } else {
            self.parent[right] = left;
            self.rank[left] += self.rank[right];
            left
        }
    }

    pub fn is_union(&mut self, left: usize, right: usize) -> bool {
        self.find(left) == self.find(right)
    }

    pub fn find(&mut self, mut node: usize) -> usize {
        let mut stack = Vec::new();
        while self.parent[node] != node {
            stack.push(node);
            node = self.parent[node];
        }
        while let Some(child) = stack.pop() {
            self.parent[child] = node
        }
        node
    }
}

impl Solution {
    fn print_maze(maze: &[Vec<i32>]) {
        for row in maze.iter().skip(1) {
            println!("{:?}", &row[1..]);
        }
        println!();
    }
    pub fn latest_day_to_cross(row: i32, col: i32, cells: Vec<Vec<i32>>) -> i32 {
        let row = row as usize;
        let col = col as usize;
        let mut union_find = UnionFind::new((row + 2) * col);

        let initial = 0;
        let last = union_find.parent.len() - 1;

        for id in initial + 1..col {
            union_find.union(initial, id);
        }
        for id in last - (col - 1)..last {
            union_find.union(id, last);
        }

        let mut result = (row * col) as i32;
        let mut maze = vec![vec![0; col]; row + 2];
        for ii in 0..col {
            maze[0][ii] = 1;
            maze[row + 1][ii] = 1;
        }
        let directions = [(-1, 0), (1, 0), (0, -1), (0, 1)];
        for cell in cells.iter().rev() {
            result -= 1;
            let x = cell[0] as usize;
            let y = (cell[1] - 1) as usize;
            let id = x * col + y;
            maze[x][y] = 1;
            for (dx, dy) in directions {
                let new_x = x as i32 + dx;
                let new_y = y as i32 + dy;
                if 0 <= new_x && new_x <= row as i32 + 1 && 0 <= new_y && new_y < col as i32 {
                    let new_x = new_x as usize;
                    let new_y = new_y as usize;
                    let neighbor_id = new_x * col + new_y;
                    if maze[new_x][new_y] == 1 {
                        union_find.union(id, neighbor_id);
                    }
                }
            }

            if union_find.is_union(initial, last) {
                break;
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::latest_day_to_cross(
                2,
                2,
                [[1, 1], [2, 1], [1, 2], [2, 2]]
                    .map(|row| row.to_vec())
                    .to_vec()
            ),
            2
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::latest_day_to_cross(
                2,
                2,
                [[1, 1], [1, 2], [2, 1], [2, 2]]
                    .map(|row| row.to_vec())
                    .to_vec()
            ),
            1
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            Solution::latest_day_to_cross(
                3,
                3,
                [
                    [1, 2],
                    [2, 1],
                    [3, 3],
                    [2, 2],
                    [1, 1],
                    [1, 3],
                    [2, 3],
                    [3, 2],
                    [3, 1]
                ]
                .map(|row| row.to_vec())
                .to_vec()
            ),
            3
        );
    }

    #[test]
    fn test4() {
        assert_eq!(
            Solution::latest_day_to_cross(
                13,
                9,
                [
                    [12, 6],
                    [3, 4],
                    [2, 9],
                    [9, 4],
                    [9, 2],
                    [6, 4],
                    [4, 4],
                    [8, 6],
                    [4, 9],
                    [5, 6],
                    [7, 5],
                    [12, 4],
                    [11, 8],
                    [3, 7],
                    [2, 6],
                    [9, 8],
                    [3, 5],
                    [13, 4],
                    [1, 3],
                    [10, 2],
                    [8, 9],
                    [6, 6],
                    [11, 7],
                    [11, 1],
                    [13, 9],
                    [12, 7],
                    [10, 7],
                    [8, 2],
                    [1, 8],
                    [7, 3],
                    [6, 5],
                    [2, 1],
                    [10, 6],
                    [4, 8],
                    [4, 2],
                    [9, 7],
                    [6, 2],
                    [3, 6],
                    [12, 2],
                    [10, 3],
                    [10, 5],
                    [9, 5],
                    [8, 8],
                    [8, 7],
                    [3, 2],
                    [13, 6],
                    [3, 1],
                    [5, 1],
                    [2, 7],
                    [8, 3],
                    [12, 5],
                    [11, 2],
                    [6, 3],
                    [1, 4],
                    [13, 3],
                    [4, 1],
                    [9, 9],
                    [7, 7],
                    [4, 3],
                    [12, 1],
                    [2, 2],
                    [7, 6],
                    [4, 6],
                    [7, 9],
                    [7, 2],
                    [3, 8],
                    [1, 6],
                    [11, 3],
                    [11, 4],
                    [5, 9],
                    [13, 8],
                    [1, 9],
                    [10, 1],
                    [9, 1],
                    [6, 1],
                    [10, 9],
                    [12, 9],
                    [11, 5],
                    [8, 1],
                    [13, 5],
                    [9, 6],
                    [13, 2],
                    [6, 8],
                    [2, 8],
                    [5, 3],
                    [3, 3],
                    [13, 1],
                    [11, 9],
                    [9, 3],
                    [2, 4],
                    [5, 2],
                    [8, 5],
                    [13, 7],
                    [12, 8],
                    [5, 5],
                    [7, 1],
                    [7, 4],
                    [2, 5],
                    [6, 9],
                    [4, 7],
                    [5, 8],
                    [1, 5],
                    [10, 8],
                    [8, 4],
                    [1, 1],
                    [3, 9],
                    [1, 2],
                    [7, 8],
                    [1, 7],
                    [6, 7],
                    [11, 6],
                    [4, 5],
                    [5, 7],
                    [2, 3],
                    [10, 4],
                    [5, 4],
                    [12, 3]
                ]
                .map(|row| row.to_vec())
                .to_vec()
            ),
            35
        );
    }
}
