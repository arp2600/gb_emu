#[derive(Debug)]
struct Regs<'a> {
    ab_raw: Box<u16>,
    a: &'a mut u8,
    b: &'a mut u8,
    ab: &'a mut u16,
}

impl<'a> Regs<'a> {
    fn new() -> Regs<'a> {
        let raw: Box<u16> = Box::new(42);

        let (a, b, ab, ab_raw): (&mut u8, &mut u8, &mut u16, Box<u16>) = unsafe {
            let x_raw = Box::into_raw(raw);
            let x_u8: *mut u8 = std::mem::transmute(x_raw);
            let a = x_u8.offset(0).as_mut().unwrap();
            let b = x_u8.offset(1).as_mut().unwrap();
            let ab = x_raw.offset(0).as_mut().unwrap();
            let ab_raw = Box::from_raw(x_raw);
            (a, b, ab, ab_raw)
        };

        Regs { ab_raw, a, b, ab }
    }
}

fn main() {
    let regs = Regs::new();
    println!("{:?}", regs);
    *regs.b = 35;
    println!("{:?}", regs);
    *regs.ab = 1234;
    println!("{:?}", regs);
}
