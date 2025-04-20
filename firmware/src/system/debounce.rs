
#[derive(Clone, Copy, defmt::Format)]
struct Item {
    value: f32,
    age: usize
}

#[derive(Clone, Copy)]
pub struct Median<const N: usize> {
    queue: [Item; N]
}

impl<const N: usize> Median<N> {
    pub fn new() -> Self {
        assert!(N > 0);
        let mut queue = [Item { value: 0.0, age: 0 }; N];
        for i in 0..N {
            queue[i] = Item { value: 0.0, age: i }
        }
        Median {
            queue
        }
    }

    pub fn insert(&mut self, value: f32) -> f32 {
        let mut index_to_pop = 0;
        for (i, Item { age, .. }) in self.queue.iter_mut().enumerate() {
            *age += 1;
            if *age == N {
                index_to_pop = i;
            }
        }
        self.queue[index_to_pop] = Item { value, age: 0 };
        
        self.sort();
        self.queue[N >> 2].value
    }

    fn sort(&mut self) {
        for i in 1..N {
            let mut j = i;
            while (j > 0) && (self.queue[j].value < self.queue[j - 1].value) {
                self.queue.swap(j, j - 1);
                j -= 1;
            }
        }
    }
}
