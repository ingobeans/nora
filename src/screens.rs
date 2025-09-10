pub struct ScreenDrawContext {}
pub struct ScreenUpdateContext {}
pub enum ScreenUpdateResult {
    /// Does nothing special
    Pass,
}

pub trait Screen {
    fn update(&mut self, ctx: ScreenUpdateContext) -> ScreenUpdateResult {
        ScreenUpdateResult::Pass
    }
    fn draw(&mut self, ctx: ScreenDrawContext) {}
}

pub struct ScreensRegistry {
    screens: Vec<Box<dyn Screen>>,
}
impl ScreensRegistry {
    pub fn new() -> Self {
        let mut screens = Vec::new();
        for id in enum_iterator::all::<ScreenID>() {
            screens.push(Self::load_screen_from_id(id));
        }

        Self { screens }
    }
    pub fn get(&self, id: ScreenID) -> &Box<dyn Screen> {
        &self.screens[id as usize]
    }
    pub fn get_mut(&mut self, id: ScreenID) -> &mut Box<dyn Screen> {
        &mut self.screens[id as usize]
    }
    fn load_screen_from_id(id: ScreenID) -> Box<dyn Screen> {
        match id {
            ScreenID::Test => Box::new(TestScreen::default()),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, enum_iterator::Sequence)]
pub enum ScreenID {
    Test,
}
struct TestScreen;
impl Screen for TestScreen {}
impl Default for TestScreen {
    fn default() -> Self {
        Self
    }
}
