pub mod header_view;
pub mod slot_view;

#[derive(Clone, Copy, PartialEq)]
pub enum PenguinView {
    Header,
    SaveSlot,
}
