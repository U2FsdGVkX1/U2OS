use core::cmp::max;

macro_rules! left_child {
    ($i: expr) => {
        $i * 2
    };
}

macro_rules! right_child {
    ($i: expr) => {
        $i * 2 + 1
    };
}

macro_rules! parent {
    ($i: expr) => {
        $i / 2
    };
}

pub struct BuddySystem {
    start: *mut u8,
    end: *mut u8,
    actual_start: *mut u8,
}

impl BuddySystem {
    const PAGE_SIZE: usize = 4096;

    #[inline]
    fn size2level(size: usize) -> u8 {
        (size.log2() + 1) as u8
    }
    #[inline]
    fn level2size(level: u8) -> usize {
        1 << (level - 1)
    }
    pub unsafe fn new(start: *mut u8, end: *mut u8) -> Self {
        let page = end.offset_from(start).unsigned_abs() / Self::PAGE_SIZE;
        let fixsize = page.next_power_of_two();

        let mut level = Self::size2level(fixsize);
        *start = level;
        for i in 1..fixsize * 2 {
            *start.add(i) = level;
            if (i + 1).is_power_of_two() {
                level -= 1;
            }
        }

        Self {
            start,
            end,
            actual_start: start,
        }
    }
    unsafe fn backtrace(&self, index: usize) {
        let mut i = index;
        let mut level = *self.start - Self::size2level(i);
        while i != 1 {
            i = parent!(i);
            let node = self.start.add(i);
            let left_node = self.start.add(left_child!(i));
            let right_node = self.start.add(right_child!(i));

            level += 1;
            *node = if *left_node == level && *right_node == level {
                level + 1
            } else {
                max(*left_node, *right_node)
            };
        }
    }
    pub unsafe fn malloc(&self, size: usize) -> *mut u8 {
        let request_size = size.next_power_of_two();
        let request_level = Self::size2level(request_size);
        let mut i = 1;
        for _ in (request_level..*self.start).rev() {
            let left_node = self.start.add(left_child!(i));
            i = if request_level <= *left_node {
                left_child!(i)
            } else {
                right_child!(i)
            };
        }

        *self.start.add(i) = 0;
        self.backtrace(i);

        let offset = i * request_size - Self::level2size(*self.start);
        self.actual_start.add(Self::PAGE_SIZE * offset)
    }
    pub unsafe fn free(&self, ptr: *mut u8) {
        let offset = ptr.offset_from(self.actual_start).unsigned_abs() / Self::PAGE_SIZE;
        let mut i = offset + Self::level2size(*self.start);
        let mut level = 1;
        while *self.start.add(i) != 0 {
            i = parent!(i);
            level += 1;
            if i == 0 { panic!("free: ptr is not in the system") }
        }
        
        *self.start.add(i) = Self::size2level(level);
        self.backtrace(i);
    }
}
