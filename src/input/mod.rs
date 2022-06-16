use winit::event::WindowEvent;

pub trait Controllable {
    fn process_events(&mut self, event: &WindowEvent) -> bool;
}