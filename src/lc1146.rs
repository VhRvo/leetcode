use std::collections::HashMap;
use std::mem;

struct SnapshotArray {
    snapshots: Vec<HashMap<i32, i32>>,
    hash_map: HashMap<i32, i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl SnapshotArray {
    /**
     * Your SnapshotArray object will be instantiated and called as such:
     * let obj = SnapshotArray::new(length);
     * obj.set(index, val);
     * let ret_2: i32 = obj.snap();
     * let ret_3: i32 = obj.get(index, snap_id);
     */
    fn new(_length: i32) -> Self {
        Self {
            snapshots: Vec::new(),
            hash_map: HashMap::new(),
        }
    }

    fn set(&mut self, index: i32, val: i32) {
        if self.snapshots.is_empty() || val != self.get(index, self.snapshots.len() as i32 - 1) {
            self.hash_map.insert(index, val);
        }
    }

    fn snap(&mut self) -> i32 {
        let hash_map = mem::take(&mut self.hash_map);
        self.snapshots.push(hash_map);
        self.snapshots.len() as i32 - 1
    }

    fn get(&self, index: i32, snap_id: i32) -> i32 {
        let mut snap_id = snap_id as usize;
        loop {
            match self.snapshots[snap_id].get(&index) {
                None => {
                    if snap_id == 0 {
                        break 0;
                    } else {
                        snap_id -= 1;
                    }
                }
                Some(&val) => {
                    break val;
                }
            }
        }
    }
}

#[test]
fn test() {}
