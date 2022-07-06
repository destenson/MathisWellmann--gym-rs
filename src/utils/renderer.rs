/// TODO
#[derive(Debug)]
pub struct Renderer<'a> {
    no_returns_render: &'a [RenderMode],
    single_render: &'a [RenderMode],
    mode: RenderMode,
    // NOTE: `render` cannot exist as a property, as it likely requires access to the enviroment using this renderer.
    // Since RenderMode would need to be a closure, the environment would need to be static which is impossible without
    // the use of the internal mutability pattern, which has its own flaws.
    //
    // As a result, we opt to pass in the closure when needed.
    // Alternatively, if we continue using a function pointer, we would need to pass the instance itself.
    render_list: Vec<Render>,
}

type RenderFn<'a> = &'a dyn Fn(RenderMode) -> Render;

impl<'a> Renderer<'a> {
    /// TODO
    pub fn new(
        mode: RenderMode,
        no_returns_render: Option<&'a [RenderMode]>,
        single_render: Option<&'a [RenderMode]>,
    ) -> Self {
        Self {
            no_returns_render: no_returns_render.unwrap_or(RenderMode::NO_RETURNS_RENDER),
            single_render: single_render.unwrap_or(RenderMode::SINGLE_RENDER),
            mode,
            render_list: Vec::new(),
        }
    }

    /// TODO
    pub fn render_step(&mut self, render: RenderFn<'a>) {
        if self.mode != RenderMode::None && !self.single_render.contains(&self.mode) {
            let render_frame = render(self.mode);
            if !self.no_returns_render.contains(&self.mode) {
                self.render_list.push(render_frame)
            }
        }
    }

    /// TODO
    pub fn get_render(&mut self, render: RenderFn<'a>) -> Option<Vec<Render>> {
        if self.single_render.contains(&self.mode) {
            Some(vec![render(self.mode)])
        } else if self.mode != RenderMode::None && !self.no_returns_render.contains(&self.mode) {
            let renders = self.render_list.clone();
            self.render_list = Vec::new();
            Some(renders)
        } else {
            None
        }
    }

    /// TODO
    pub fn reset(&mut self) {
        self.render_list = Vec::new();
    }
}

/// TODO
pub type RenderFrame = [[[usize; 3]; 255]; 255];

/// TODO
#[derive(PartialEq, PartialOrd, Debug, Clone, Copy)]
pub enum RenderMode {
    /// TODO
    Human,
    /// TODO
    SingleRgbArray,
    /// TODO
    RgbArray,
    /// TODO
    Ansi,
    /// TODO
    None,
}

impl RenderMode {
    const NO_RETURNS_RENDER: &'static [RenderMode] = &[RenderMode::Human];
    const SINGLE_RENDER: &'static [RenderMode] = &[RenderMode::SingleRgbArray];
}

/// TODO
#[derive(PartialEq, PartialOrd, Debug, Clone)]
pub enum Render {
    /// TODO
    Human,
    /// TODO
    SingleRgbArray(RenderFrame),
    /// TODO
    RgbArray(Vec<RenderFrame>),
    /// TODO
    Ansi(Vec<String>),
    /// TODO
    None,
}
