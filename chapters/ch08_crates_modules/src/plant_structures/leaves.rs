//! `plant_structures` 的 `leaves` 子模块。

/// 叶：光合作用。
#[derive(Debug, Clone)]
pub struct Leaf {
    pub area_cm2: f32,
}

impl Leaf {
    pub fn new(area_cm2: f32) -> Self {
        Self { area_cm2 }
    }

    /// 模拟光合作用产物。
    pub fn photosynthesize(&self, light_intensity: f32) -> f32 {
        self.area_cm2 * light_intensity * 0.01
    }
}
