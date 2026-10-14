use dioxus::prelude::*;

pub trait Negateable {
    fn negate(&mut self);
}

impl Negateable for Signal<bool> {
    fn negate(&mut self) {
        let inv = !*self.read();
        self.set(inv);
    }
}

pub trait Optional<T> {
    fn call_if_some(&self, value: T);
}

impl<T: 'static> Optional<T> for Option<EventHandler<T>> {
    fn call_if_some(&self, value: T) {
        if let Some(handler) = self {
            handler(value);
        }
    }
}

pub trait HasClass {
    fn class(&self) -> &'static str;
}

pub trait HasIcon {
    fn icon(&self) -> Element;
}
