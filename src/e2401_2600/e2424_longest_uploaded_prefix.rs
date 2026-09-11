#[derive(Debug)]
struct LUPrefix {
    counts: Vec<i32>,
    map: Vec<usize>,
}

impl LUPrefix {
    fn new(n: i32) -> Self {
        let n = n as usize;
        Self {
            counts: vec![0; n+2],
            map: (0..n+2).collect::<Vec<_>>(),
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.map[x] != x {
            self.map[x] = self.find(self.map[x]);
        }

        self.map[x]
    }

    fn upload(&mut self, video: i32) {
        let video = video as usize;
        if self.counts[video] > 0 {
            return;
        }

        self.counts[video] = 1;
        if self.counts[video+1] > 0 {
            self.counts[video] += self.counts[video+1];
            self.map[video+1] = video;
        }

        if self.counts[video-1] > 0 {
            let root = self.find(video-1);
            self.counts[root] += self.counts[video];
            self.map[video] = root;
        }
    }

    fn longest(&self) -> i32 {
        self.counts[1]
    }
}

pub fn main() {
    let mut lu_prefix = LUPrefix::new(4);
    lu_prefix.upload(3);
    println!("{}", lu_prefix.longest());
    lu_prefix.upload(1);
    println!("{}", lu_prefix.longest());
    lu_prefix.upload(2);
    println!("{}", lu_prefix.longest());

    println!("{:?}", lu_prefix);
}
