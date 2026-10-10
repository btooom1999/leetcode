const MOD: i64 = 1_000_000_007;

#[derive(Debug)]
struct Fancy {
    i: usize,
    n: usize,
    tree: Vec<(i64, i64, i64)>,
}

impl Fancy {
    fn new() -> Self {
        let n = 1 << 20;
        Self { i: 0, n, tree: vec![(0, 1, 0); n*2-1] }
    }

    fn append(&mut self, val: i32) {
        self.tree[self.i+self.n-1] = (val as i64, 1, 0);
        self.i += 1;
    }

    fn push_down(&mut self, pos: usize, is_leaf: bool) {
        let (_, a1, b1) = self.tree[pos];
        if a1 != 1 || b1 != 0 {
            self.tree[pos].0 = (a1 * self.tree[pos].0 % MOD + b1) % MOD;

            if !is_leaf {
                let (_, a2, b2) = self.tree[pos*2+1];
                self.tree[pos*2+1] = (self.tree[pos*2+1].0, a1 * a2 % MOD, (a1 * b2 % MOD + b1) % MOD);

                let (_, a2, b2) = self.tree[pos*2+2];
                self.tree[pos*2+2] = (self.tree[pos*2+2].0, a1 * a2 % MOD, (a1 * b2 % MOD + b1) % MOD);
            }

            self.tree[pos].1 = 1;
            self.tree[pos].2 = 0;
        }
    }

    fn update_lazy(&mut self, l: usize, h: usize, ql: usize, qh: usize, pos: usize, delta: i64, is_mul: bool) {
        if l > qh || h < ql {
            return;
        }

        self.push_down(pos, l == h);

        if ql <= l && h <= qh {
            if is_mul {
                self.tree[pos] = (self.tree[pos].0, self.tree[pos].1 * delta % MOD, self.tree[pos].2 * delta % MOD);
            } else {
                self.tree[pos].2 = (self.tree[pos].2+delta) % MOD;
            }
            return;
        }

        let m = (l+h)/2;
        self.update_lazy(l, m, ql, qh, pos*2+1, delta, is_mul);
        self.update_lazy(m+1, h, ql, qh, pos*2+2, delta, is_mul);
    }

    fn add_all(&mut self, inc: i32) {
        self.update_lazy(0, self.n-1, 0, self.i-1, 0, inc as i64, false);
    }

    fn mult_all(&mut self, m: i32) {
        self.update_lazy(0, self.n-1, 0, self.i-1, 0, m as i64, true);
    }

    fn update_num(&mut self, l: usize, h: usize, ql: usize, qh: usize, pos: usize) {
        if l > qh || h < ql {
            return;
        }

        self.push_down(pos, l == h);

        if ql <= l && h <= qh {
            return;
        }

        let m = (l+h)/2;
        self.update_num(l, m, ql, qh, pos*2+1);
        self.update_num(m+1, h, ql, qh, pos*2+2);
    }

    fn get_index(&mut self, idx: i32) -> i32 {
        if idx as usize >= self.i { return -1; }
        self.update_num(0, self.n-1, idx as usize, idx as usize, 0);
        self.tree[idx as usize + self.n- 1].0 as i32
    }
}

pub fn main() {
    let mut fancy = Fancy::new();
    fancy.append(2);
    fancy.add_all(3);
    fancy.mult_all(2);
    println!("{}", fancy.get_index(0));
    // println!("{}", fancy.get_index(0));
}
