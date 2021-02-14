#![allow(warnings)]

#[derive(Debug)]
struct FontLoader(String);

#[derive(Debug)]
struct Font<'a>(&'a str);

impl FontLoader {
    fn load(&self) -> Font<'_> {
        Font(&self.0)
    }
}

struct Window;

#[derive(Debug)]
struct Phi<'a> {
    loader: &'a FontLoader,
    font: Option<Font<'a>>,
}

impl<'a> Phi<'a> {
    fn do_the_thing(&mut self) {
        let font = self.loader.load();
        self.font = Some(font);
    }
}

fn main() {
    let ldr = FontLoader("courier".to_string());
    let ldr2 = FontLoader("helvetica".to_string());

    let mut phi = Phi {
        loader: &ldr,
        font: None,
    };
    phi.do_the_thing();

    phi.loader = &ldr2;

    dbg!(&phi);

    drop(ldr);
    //dbg!(&phi);

}
