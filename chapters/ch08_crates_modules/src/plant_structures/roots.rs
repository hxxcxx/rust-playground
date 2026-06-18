//! `plant_structures` 的 `roots` 子模块。

/// 根：吸收水分与矿物质。
#[derive(Debug, Clone)]
pub struct Root {
    pub depth_cm: f32,
}

impl Root {
    pub fn new(depth_cm: f32) -> Self {
        Self { depth_cm }
    }

    /// 返回根是否足够深以支撑大树。
    pub fn is_deep_enough(&self) -> bool {
        self.depth_cm > 50.0
    }
}
