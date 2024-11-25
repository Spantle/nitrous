use bank::VramBank;

mod bank;
mod models;

#[derive(Default)]
pub struct VramBanks {
    pub a: VramBank<0>,
    pub b: VramBank<1>,
    pub c: VramBank<2>,
    pub d: VramBank<3>,
    pub e: VramBank<4>,
    pub f: VramBank<5>,
    pub g: VramBank<6>,
    pub h: VramBank<7>,
    pub i: VramBank<8>,
}

impl VramBanks {
    pub fn new_fake() -> Self {
        Self {
            a: VramBank::new_fake(),
            b: VramBank::new_fake(),
            c: VramBank::new_fake(),
            d: VramBank::new_fake(),
            e: VramBank::new_fake(),
            f: VramBank::new_fake(),
            g: VramBank::new_fake(),
            h: VramBank::new_fake(),
            i: VramBank::new_fake(),
        }
    }

    pub fn read_slice<const T: usize>(&self, addr: usize) -> Option<[u8; T]> {
        let (a_s, a) = self.a.read_slice::<T>(addr);
        let (b_s, b) = self.b.read_slice::<T>(addr);
        let (c_s, c) = self.c.read_slice::<T>(addr);
        let (d_s, d) = self.d.read_slice::<T>(addr);
        let (e_s, e) = self.e.read_slice::<T>(addr);
        let (f_s, f) = self.f.read_slice::<T>(addr);
        let (g_s, g) = self.g.read_slice::<T>(addr);
        let (h_s, h) = self.h.read_slice::<T>(addr);
        let (i_s, i) = self.i.read_slice::<T>(addr);

        let success = a_s | b_s | c_s | d_s | e_s | f_s | g_s | h_s | i_s;
        if !success {
            return None;
        }

        let mut result = [0; T];
        for x in 0..T {
            result[x] = a[x] | b[x] | c[x] | d[x] | e[x] | f[x] | g[x] | h[x] | i[x];
        }

        Some(result)
    }

    pub fn write_slice<const T: usize>(&mut self, addr: usize, value: [u8; T]) -> bool {
        let a = self.a.write_slice::<T>(addr, value);
        let b = self.b.write_slice::<T>(addr, value);
        let c = self.c.write_slice::<T>(addr, value);
        let d = self.d.write_slice::<T>(addr, value);
        let e = self.e.write_slice::<T>(addr, value);
        let f = self.f.write_slice::<T>(addr, value);
        let g = self.g.write_slice::<T>(addr, value);
        let h = self.h.write_slice::<T>(addr, value);
        let i = self.i.write_slice::<T>(addr, value);

        a | b | c | d | e | f | g | h | i
    }
}
