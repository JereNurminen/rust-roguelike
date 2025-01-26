pub enum Animation {
    Move {
        start: (f32, f32),
        end: (f32, f32),
        progress: f32,
        duration: f32,
    },
    Idle {
        base_pos: (f32, f32),
        time: f32,
    },
}
