pub trait Bite {
    fn bite(self: &mut Self);
}

#[derive(Debug)] // This enables using the debugging format string "{:?}"
pub struct Carrot {
    pub percent_left: f32,
}

#[derive(Debug)] // include this line right before your struct definition
pub struct Grapes {
    pub amount_left: u32,
}

impl Bite for Carrot {
    fn bite(self: &mut Self) {
        // Eat 20% of the remaining carrot. It may take awhile to eat it all...
        self.percent_left *= 0.8;
    }
}

impl Bite for Grapes {
    fn bite(self: &mut Self) {
        //Eat 1 of the grapes.
        self.amount_left -= 1;
    }
}

pub fn bunny_nibbles<T: Bite>(item: &mut T) {
    item.bite();
    item.bite();
    item.bite();
}