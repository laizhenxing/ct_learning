use crate::pb::Spec;
use image::ImageOutputFormat;

mod photon;
pub use photon::Photon;

pub trait Engine {
    // 对 engine 按照specs一系列有序的处理
    fn apply(&mut self, specs: &[Spec]);
    // 从 engine 中生成目标图片，注意用的是 self, 而非 &self
    fn generate(self, format: ImageOutputFormat) -> Vec<u8>;
}

// SpecTransform: 如果又更多的 spec, 只需要实现它即可
pub trait SpecTransform<T> {
    fn transform(&mut self, op: T);
}
