struct Solution;

mod inefficient {
    // # 效率问题分析
    //
    // ## 朴素的选择想法
    //
    // 先从最直接的思路出发：
    // 每一列都可以拆成若干个连续 1 的 segment，如果想拼出一个全 1 子矩阵，
    // 就是在若干列里各选一个 segment，使这些 segment 的公共交集非空。
    // 这里特意强调“公共交集非空”，是因为如果公共交集已经为空，
    // 那么从状态表示上说它仍然可以被描述成一种“选择结果”，只是对应的高度为 0，
    // 不可能贡献任何有效面积。
    // 因此，在搜索过程中只保留公共交集非空的选择，其实是一种很自然的剪枝：
    // 一旦交集变空，这条分支后面就不值得再继续扩展了。
    // 一旦公共交集是 [start, end)，那么：
    // 这里的 [start, end) 只是这些被选中 segments 的公共交集，
    // 它不一定等于其中任何一条完整的 segment，很多时候只是它们重叠出来的一部分。
    //   1. 高度就是 end - start
    //   2. 宽度就是选中的列数
    //   3. 面积就是 (end - start) * 列数
    //
    // 因而，问题可以理解为：
    // 不断尝试把新的 segment 加入当前选择，只要这些被选中的 segment 仍有公共交集，
    // 就得到一个合法矩阵，并尝试用它更新答案。
    //
    // ## 状态定义
    //
    // `matries` 中每个 `NonEmptyMatrix` 是一个三元组：
    //
    //   ( existed: HashSet<列下标>,  leftest_start: i32,  leftest_end: i32 )
    //
    // 含义：存在一组列（`existed`），它们各自的连续 1 区间都包含行区间
    // `[leftest_start, leftest_end]`。
    // 也就是说，这个行区间描述的是当前所有已选 segments 的公共交集，
    // 而不是某一列上某一条原始 segment 本身。
    //
    // ## 状态如何增长（以 3 列全 1 矩阵为例，3 个 segment 行区间均为 [0,3]）
    //
    // 初始：matries = []
    //
    // 处理 seg(col=0)：遍历 matries（空，无克隆）。新增自身单例。
    //   matries = [
    //     ① {existed:{0}, row:[0,3]}
    //   ]  共 1 个
    //
    // 处理 seg(col=1)：遍历 matries 中 1 个状态：
    //   - ① {0} 不含 col=1，行区间有交集 → 克隆产生 {0,1}
    //   新增自身单例 {1}。
    //   matries = [
    //     ① {existed:{0},   row:[0,3]}
    //     ② {existed:{0,1}, row:[0,3]}   ← 由①克隆
    //     ③ {existed:{1},   row:[0,3]}   ← 单例
    //   ]  共 3 个
    //
    // 处理 seg(col=2)：遍历 matries 中 3 个状态：
    //   - ① {0}   不含 col=2 → 克隆产生 {0,2}
    //   - ② {0,1} 不含 col=2 → 克隆产生 {0,1,2}
    //   - ③ {1}   不含 col=2 → 克隆产生 {1,2}
    //   新增自身单例 {2}。
    //   matries = [
    //     ① {0}     ② {0,1}   ③ {1}      ← 旧的，永不删除
    //     ④ {0,2}   ⑤ {0,1,2} ⑥ {1,2}  ⑦ {2}
    //   ]  共 7 个
    //
    // ## 爆炸规律
    //
    // 每轮新增 = 当前 matries 大小（旧状态各克隆一个）+ 1（单例）：
    //
    //   col=0: 0 旧 + 1 单例 = 1
    //   col=1: 1 旧 + 1 单例 = 3   （= 2^2 - 1）
    //   col=2: 3 旧 + 1 单例 = 7   （= 2^3 - 1）
    //   col=k: 2^k - 1 旧 + 1 单例 = 2^(k+1) - 1
    //
    // 最终 matries 恰好枚举了所有 2^n - 1 个非空列子集。
    //
    // ## 根本原因
    //
    // `existed` 记录了"哪些具体列被选入"，使得 {0,2} 和 {1,2} 成为两个不同
    // 的状态，即便它们宽度相同、行区间也一样。算法对所有可能的列子集分别维护
    // 一条状态，且旧状态从不删除，所以必然指数爆炸。
    //
    // ## 为什么还要对 segments 排序
    //
    // 这里按 segment 长度从大到小排序，并不是算法正确性的必要条件；
    // 即使不排序，这个写法最终仍然会枚举到同样的状态。
    // 这样做更多是为了让状态扩展的过程更直观：
    //   1. 先处理较长的 segment，比较像是先搭出“较大的候选骨架”
    //   2. 后处理较短的 segment，再不断把已有状态的公共交集收紧
    //   3. 因而更容易从直觉上理解“往状态里继续加 segment，只会让可行行区间变小”
    //
    // 换句话说，排序主要是为了帮助组织搜索顺序和理解过程，
    // 真正导致低效的根源并不在排序本身，而在于状态数量会按列子集指数爆炸。
    use std::collections::HashSet;

    struct Segment {
        column: i32,
        start: i32,
        end: i32,
    }

    fn from(column: i32, length: i32, end: i32) -> Segment {
        Segment {
            column,
            start: end - length,
            end,
        }
    }

    struct NonEmptyMatrix {
        existed: HashSet<i32>,
        leftest_start: i32,
        leftest_end: i32,
    }

    fn interval(left: (i32, i32), right: (i32, i32)) -> Option<(i32, i32)> {
        let up = left.0.max(right.0);
        let down = left.1.min(right.1);
        if up <= down {
            Some((up, down))
        } else {
            None
        }
    }

    pub fn largest_submatrix(matrix: Vec<Vec<i32>>) -> i32 {
        let m = matrix.len();
        let n = matrix[0].len();
        let segments = {
            let mut column_accumuators = vec![0; n];
            let mut segments = Vec::with_capacity(n);
            for ii in 0..m {
                for jj in 0..n {
                    if matrix[ii][jj] == 1 {
                        column_accumuators[jj] += 1;
                    } else if column_accumuators[jj] != 0 {
                        segments.push(from(jj as i32, column_accumuators[jj], ii as i32));
                        column_accumuators[jj] = 0;
                    }
                }
            }
            for jj in 0..n {
                if column_accumuators[jj] != 0 {
                    segments.push(from(jj as i32, column_accumuators[jj], m as i32));
                }
            }
            segments.sort_by(|left, right| {
                (left.end - left.start)
                    .cmp(&(right.end - right.start))
                    .reverse()
            });
            segments
        };
        let mut matries = Vec::<NonEmptyMatrix>::new();
        let mut result = 0;
        for segment in segments.iter() {
            let mut current_matries_result = Vec::<NonEmptyMatrix>::new();
            for matrix in matries.iter() {
                if matrix.existed.contains(&segment.column) {
                    continue;
                }
                if let Some((leftest_start, leftest_end)) = interval(
                    (matrix.leftest_start, matrix.leftest_end),
                    (segment.start, segment.end),
                ) {
                    let mut existed = matrix.existed.clone();
                    existed.insert(segment.column);
                    result = result.max((leftest_end - leftest_start) * (existed.len() as i32));
                    current_matries_result.push(NonEmptyMatrix {
                        existed,
                        leftest_start,
                        leftest_end,
                    });
                }
            }
            let mut existed = HashSet::new();
            existed.insert(segment.column);

            current_matries_result.push(NonEmptyMatrix {
                existed,
                leftest_start: segment.start,
                leftest_end: segment.end,
            });
            result = result.max(segment.end - segment.start);
            matries.extend(current_matries_result);
        }
        result
    }
}

mod improved1 {
    // # 从 inefficient 到 improved 的推理过程
    //
    // ## My idea
    // 既然 (existed, start, end) 已经能够完整描述一组线段的选择结果，
    // 那么就可以反过来直接枚举 start 和 end，
    // 再判断每一列是否存在某个 segment 覆盖这个区间。
    // 由于同一列中的各个 segment 彼此不重叠，
    // 对于固定的区间 [start, end)，一列中至多只有一个 segment 能覆盖它，
    // 所以可以按列维护各自的 segments，而不必再用 existed 哈希表记录“具体选了哪些列”。
    // 这样就把原先按列子集展开的指数级搜索，改写成了按行区间枚举的多项式级算法。
    //
    // ## 为什么不再需要 existed
    //
    // 前面的 inefficient 已经说明，状态爆炸的根源在于把“具体选了哪些列”也编码进了状态。
    // 但对固定的区间 [start, end) 来说，面积只取决于两件事：
    //   1. 高度 end - start
    //   2. 有多少列能覆盖这个区间
    // 并不取决于这些列的具体身份。
    // 因此，对固定的 [start, end)，最优做法就是把所有能覆盖它的列全部纳入，
    // 不再需要枚举列子集，也不再需要维护 existed。
    //
    // ## 直接得到的算法
    //
    //   1. 枚举所有行区间 [start, end)
    //   2. 对每一列判断是否存在某个 segment 完整覆盖该区间，若有则 count++
    //   3. 用 count × (end - start) 更新答案
    //
    // 由于同一列中的 segment 互不重叠，所以将 segments 按列分组存储即可。
    //
    // ## 复杂度对比
    //
    //   inefficient: O(2^n)       — 枚举所有列子集
    //   improved1:   O(m² · n · k) — 枚举行区间 × 扫描各列
    //
    // 从指数级降到多项式级。
    pub fn largest_submatrix(matrix: Vec<Vec<i32>>) -> i32 {
        let m = matrix.len();
        let n = matrix[0].len();
        let column_segments = {
            let mut column_accumuators = vec![0; n];
            let mut column_segments = vec![Vec::new(); n];
            for ii in 0..m {
                for jj in 0..n {
                    if matrix[ii][jj] == 1 {
                        column_accumuators[jj] += 1;
                    } else if column_accumuators[jj] != 0 {
                        column_segments[jj].push((ii - column_accumuators[jj], ii));
                        column_accumuators[jj] = 0;
                    }
                }
            }
            for jj in 0..n {
                if column_accumuators[jj] != 0 {
                    column_segments[jj].push((m - column_accumuators[jj], m));
                }
            }
            column_segments
        };
        let mut result = 0;
        for start in 0..m {
            for end in start + 1..=m {
                let mut count = 0;
                for jj in 0..n {
                    if column_segments[jj]
                        .iter()
                        .find(|(segment_start, segment_end)| {
                            *segment_start <= start && end <= *segment_end
                        })
                        .is_some()
                    {
                        count += 1;
                    }
                }
                result = result.max(count * (end - start));
            }
        }
        result as i32
    }
}

mod improved2 {
    // 在 improved1 版本中，我进一步注意到：
    // 固定 start 后，随着 end 增大，能够覆盖区间 [start, end) 的列只会越来越少。
    // 而对某一列来说，真正有用的并不是这一列的全部 segments，
    // 而只是“覆盖 start 的那个 segment 最多还能向下延伸多远”。
    // 这是因为同一列中的各个 segment 彼此不重叠，所以对固定的 start，
    // 每列至多只有一个 segment 可能参与答案。
    // 因此，可以把这条唯一相关的信息预处理成高度：
    //   heights[start][col] = 从 start 行出发，列 col 向下连续 1 的最大长度
    // 这样就不需要在遍历 end 时反复查找 segment 了。
    // 对固定的 start，把各列能够提供的高度排序后，
    // 就可以直接枚举“选多少列”来计算该 start 下的最大面积。
    pub fn largest_submatrix(matrix: Vec<Vec<i32>>) -> i32 {
        let m = matrix.len();
        let n = matrix[0].len();
        let mut heights = {
            let mut column_heights = vec![0; n];
            let mut heights = vec![vec![0; n]; m];
            for ii in (0..m).rev() {
                for jj in 0..n {
                    if matrix[ii][jj] == 1 {
                        column_heights[jj] += 1;
                    } else {
                        column_heights[jj] = 0;
                    }
                    heights[ii][jj] = column_heights[jj];
                }
            }
            heights
        };
        let result = {
            let mut result = 0;
            for ii in 0..m {
                heights[ii].sort();
                let mut count = 0;
                for jj in (0..n).rev() {
                    count += 1;
                    result = result.max(count * heights[ii][jj]);
                }
            }
            result
        };
        result
    }
}

mod optimal {
    // # 从 improved2 到 optimal 的推理过程
    //
    // improved2 的瓶颈在于：
    // 对每个 start，都要把 heights[start] 排序一次，因此复杂度是 O(mn log n)。
    //
    // 这里继续沿用 improved2 的视角：
    //   column_heights[col] = 从当前 start 行出发，列 col 向下连续 1 的最大长度
    //
    // 当 start 从下往上移动一行时，每一列的高度只会发生两种变化：
    //   1. 如果 matrix[start][col] == 0，高度直接变成 0
    //   2. 如果 matrix[start][col] == 1，高度就在原来的基础上 +1
    //
    // 关键在于，这种更新不会任意打乱列高的相对顺序：
    // 所有变成 0 的列统一放到前面；所有仍为正数的列统一放到后面，
    // 而后者彼此之间只是整体 +1，原有大小关系不会改变。
    //
    // 因此，如果上一轮列下标已经按高度升序排好，
    // 那这一轮无需重新排序，只要做一次稳定划分：
    //   - 当前行为 0 的列放前面
    //   - 当前行为 1 的列放后面
    // 就能继续保持“按当前高度升序排列”的不变量。
    //
    // 有了这个有序顺序后，从大到小扫描这些列：
    //   - width 表示当前已经选了多少列
    //   - column_heights[col] 表示这 width 列里的最小高度
    //   - 因而面积候选就是 width * column_heights[col]
    //
    // 这样就把 improved2 中每一行的 O(n log n) 排序，优化成了 O(n) 的稳定重排，
    // 总复杂度降为 O(mn)。
    pub fn largest_submatrix(matrix: Vec<Vec<i32>>) -> i32 {
        let m = matrix.len();
        let n = matrix[0].len();
        let mut column_heights = vec![0; n];
        // 列下标始终按当前 column_heights 升序排列。
        let mut sorted_column_indexes = (0..n).collect::<Vec<usize>>();
        // 复用临时缓冲区，避免每一轮重复分配。
        let mut zero_columns = Vec::with_capacity(n);
        let mut positive_columns = Vec::with_capacity(n);
        let mut result = 0;

        for row in (0..m).rev() {
            // drain(..) 会把上一轮的有序结果整体移出并清空 Vec：
            //   - len 会变成 0
            //   - capacity 通常保持不变，因此下一轮仍可复用这块缓冲区
            // 这样既能拿到元素所有权，又能保留 sorted_column_indexes 的底层分配以便回填。
            // 按上一轮的升序顺序扫描，然后稳定地拆成 0 和正高度两段。
            for column in sorted_column_indexes.drain(..) {
                if matrix[row][column] == 0 {
                    column_heights[column] = 0;
                    zero_columns.push(column);
                } else {
                    column_heights[column] += 1;
                    positive_columns.push(column);
                }
            }

            let mut width = 0;
            for &column in positive_columns.iter().rev() {
                width += 1;
                // 当前扫过的 width 列中，column_heights[column] 是最小高度。
                result = result.max(width * column_heights[column]);
            }

            // append 会把两个缓冲区中的元素整体搬回 sorted_column_indexes：
            //   - zero_columns / positive_columns 的 len 会变成 0
            //   - 它们的 capacity 通常保持不变，因此下一轮可以继续复用
            //   - sorted_column_indexes 如果容量不够，可能在这里扩容；够的话就直接复用原缓冲区
            sorted_column_indexes.append(&mut zero_columns);
            sorted_column_indexes.append(&mut positive_columns);
        }

        result as i32
    }
}

impl Solution {
    pub fn largest_submatrix(matrix: Vec<Vec<i32>>) -> i32 {
        // inefficient::largest_submatrix(matrix)
        // improved1::largest_submatrix(matrix)
        // improved2::largest_submatrix(matrix)
        optimal::largest_submatrix(matrix)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::largest_submatrix(vec![vec![0, 0, 1], vec![1, 1, 1], vec![1, 0, 1]]),
            4
        );
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::largest_submatrix(vec![vec![1, 0, 1, 0, 1],]), 3);
    }

    #[test]
    fn test3() {
        assert_eq!(
            Solution::largest_submatrix(vec![vec![1, 1, 0], vec![1, 0, 1]]),
            2
        );
    }
}
