use crate::chunk::{Values, ObjString};
use std::alloc::{self, Layout};

pub struct Table {
    count: usize,
    capacity: usize,
    entries: *mut Entry,
}

pub struct Entry {
    key: *mut ObjString,
    value: Values,
 }

impl Table {
    pub fn new() -> Self {
        Self {
            count: 0,
            capacity: 0,
            entries: std::ptr::null_mut(),
        }
    }

    pub fn table_set(&mut self, key: *mut ObjString, value: Values) -> bool {
        // Load factor: 0.75
        if (self.count + 1) * 3 > self.capacity * 4 {
            let new_capacity = if self.capacity < 8 { 8 } else { self.capacity * 2 };
            self.adjust_capacity(new_capacity);
        }

        unsafe {
            let entry: *mut Entry = self.find_entry(self.entries, key);
            let is_new_key: bool = (*entry).key.is_null();
            if is_new_key && (*entry).value == Values::Nil { self.count += 1; }

            (*entry).key = key;
            (*entry).value = value;
            return is_new_key;
        }
    }

    pub fn find_entry(&self, entries: *mut Entry, key: *mut ObjString) -> *mut Entry {
        let capacity = self.capacity;
        unsafe {
            let mut index: usize = ((*key).hash as usize) % capacity;
            let mut tombstone: *mut Entry = std::ptr::null_mut();

            loop {
                let entry = entries.add(index);
                let entry_key = (*entry).key;

                if entry_key.is_null() {
                    match (*entry).value {
                        Values::Nil => return if !tombstone.is_null() { tombstone } else { entry },
                        Values::Tombstone => if tombstone.is_null() { tombstone = entry },
                        _ => {},
                    }
                } else if ObjString::equals(entry_key, key) { return entry; }

                index = (index + 1) % capacity;
            }
        }
    }
    
    pub fn adjust_capacity(&mut self, capacity: usize) {
        unsafe {
            let layout = Layout::array::<Entry>(capacity).unwrap();
            let entries: *mut Entry = alloc::alloc(layout) as *mut Entry;
            for i in 0..capacity {
                (*entries.add(i)).key = std::ptr::null_mut();
                (*entries.add(i)).value = Values::Nil;
            }
            self.count = 0;

            for i in 0..self.capacity {
                let old_entry: *mut Entry = self.entries.add(i);
                if (*old_entry).key.is_null() { continue; }

                let dest = self.find_entry(entries, (*old_entry).key);
                (*dest).key = (*old_entry).key;
                (*dest).value = (*old_entry).value;
                self.count += 1;
            }

            let old_layout = Layout::array::<Entry>(self.capacity).unwrap();
            alloc::dealloc(self.entries as *mut u8, old_layout);

            self.entries = entries;
            self.capacity = capacity;
        }
    }

    pub fn table_add_all(&self, to: &mut Table) {
        unsafe {
            for i in 0..self.capacity {
                let entry = self.entries.add(i);
                if !(*entry).key.is_null() {
                    to.table_set((*entry).key, (*entry).value);
                }
            }
        }
    }

    pub fn table_get(&self, key: *mut ObjString) -> Option<Values> {
        if self.count == 0 {
            return None;
        }

        unsafe {
            let entry = self.find_entry(self.entries, key);
            if (*entry).key.is_null() {
                None
            } else {
                Some((*entry).value.clone())
            }
        }
    }

    pub fn table_delete(&mut self, key: *mut ObjString) -> bool {
        unsafe {
            if self.count == 0 {
                return false;
            }

            let entry = self.find_entry(self.entries, key);
            if (*entry).key.is_null() {
                return false;
            }

            (*entry).key = std::ptr::null_mut();
            // add Tombstone
            (*entry).value = Values::Tombstone;
            true
        }
    }

    pub fn free_table(&mut self) {
        unsafe {
            if !self.entries.is_null() {
                let layout = Layout::array::<Entry>(self.capacity).unwrap();
                alloc::dealloc(self.entries as *mut u8, layout);
            }
            self.entries = std::ptr::null_mut();
            self.count = 0;
            self.capacity = 0;
        }
    }
}

impl Drop for Table {
    fn drop(&mut self) {
        self.free_table();
    }
}

impl ObjString {
    pub fn equals(a: *mut ObjString, b: *mut ObjString) -> bool {
        unsafe {
            if (*a).length != (*b).length { return false; }
            std::slice::from_raw_parts((*a).chars, (*a).length) ==
                std::slice::from_raw_parts((*b).chars, (*b).length)
        }
    }
}

