use std::{default::Default, sync::Arc};

fn main() {
    pub trait Hittable {
        fn hit(&self) -> Option<()> {
            None
        }
    }
    impl Default for dyn Hittable {
        fn default() -> Self {
            Self::default()
        }
    }
    fn a(b: Arc<dyn Hittable>) -> Option<()> {
        b.hit()
    }
    a(Arc::default());
}
