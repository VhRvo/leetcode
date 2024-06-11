use std::collections::HashMap;

struct SnapshotArray {
    cur_snap_id: i32,
    history: HashMap<i32, Vec<(i32, i32)>>, // 每个 index 的历史修改记录
}

impl SnapshotArray {
    fn new(_: i32) -> Self {
        Self {
            cur_snap_id: 0,
            history: HashMap::new(),
        }
    }

    fn set(&mut self, index: i32, val: i32) {
        self.history
            .entry(index)
            .or_default()
            .push((self.cur_snap_id, val));
    }

    fn snap(&mut self) -> i32 {
        self.cur_snap_id += 1;
        self.cur_snap_id - 1
    }

    fn get(&self, index: i32, snap_id: i32) -> i32 {
        if let Some(h) = self.history.get(&index) {
            let j = h.partition_point(|&(id, _)| id <= snap_id);
            if j > 0 {
                return h[j - 1].1;
            }
        }
        0
    }
}
