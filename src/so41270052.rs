// https://stackoverflow.com/questions/41270052/cannot-infer-an-appropriate-lifetime-for-autoref-due-to-conflicting-requirements/41271422#41271422

struct FontLoader(String);
struct Font<'a>(&'a str);

impl FontLoader {
    fn load(&self) -> Font {
        Font(&self.0)
    }
}

struct Window;

struct Phi<'window> {
    window: &'window Window,
    loader: FontLoader,
    font: Option<Font<'window>>,
}

/*
impl<'window> Phi<'window> {
    fn do_the_thing(&mut self) {
        let font = self.loader.load();
        self.font = Some(font);
    }
}
*/

fn main() {
    let window = Window {};
    let loader = FontLoader("courier".to_string());
    let mut phi = Phi {
        window: &window,
        loader,
        font: None,
    };

    let font = phi.loader.load();
    phi.font = Some(font);
}
