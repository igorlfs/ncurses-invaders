pub trait Object {
    fn pos(&self) -> (i32, i32);
    fn char(&self) -> u32;
    fn color(&self) -> i16;
}
