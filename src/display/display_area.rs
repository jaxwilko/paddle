use crate::{
    error::NutsCheck,
    graphics::AbstractMesh,
    quicksilver_compat::{Background, Shape},
    Display, ErrorMessage, Rectangle, Tessellate, Transform, Vector,
};
use div::DivHandle;
use web_sys::Element;

pub struct DisplayArea {
    /// in game coordinates (0|0 is at the top left of display)
    region: Rectangle,
    /// the full display
    display: Display,
    /// Div element that covers the display area, which is used for displaying HTML
    div: DivHandle,
}

impl DisplayArea {
    /// Select an area inside the full display. Ara specified in game coordinates.
    pub fn select(&mut self, rect: Rectangle, div: DivHandle) -> &mut Self {
        self.region = rect;
        self.div = div;
        self
    }
    /// The full display area.
    pub fn full(&self) -> &Display {
        &self.display
    }
    /// The full display area.
    pub fn full_mut(&mut self) -> &mut Display {
        &mut self.display
    }
    /// Converts from coordinates used inside the frame (where 0,0 is at the top left corner of the frame area)
    /// to a coordinate system covering the full display
    pub fn frame_to_display_coordinates(&self) -> Transform {
        Transform::translate(self.region.pos)
    }
    pub fn display_to_frame_coordinates(&self) -> Transform {
        Transform::translate(-self.region.pos)
    }
    /// In game coordinates (covering full display)
    pub fn is_inside(&self, display_coordinates: impl Into<Vector>) -> bool {
        self.region.contains(display_coordinates)
    }
    /// Draw a Drawable to the window, which will be finalized on the next flush
    pub fn draw<'a>(&'a mut self, draw: &impl Tessellate, bkg: impl Into<Background<'a>>) {
        self.display
            .draw_ex(draw, bkg.into(), self.frame_to_display_coordinates(), 0);
    }
    /// Fills selected area with the given color (or image)
    pub fn fill<'a>(&'a mut self, bkg: impl Into<Background<'a>>) {
        let region = Rectangle::new_sized(self.region.size);
        self.draw(&region, bkg);
    }
    /// Draw a Drawable to the window with more options provided (draw exhaustive)
    pub fn draw_ex<'a>(
        &'a mut self,
        draw: &impl Tessellate,
        bkg: impl Into<Background<'a>>,
        trans: Transform,
        z: i16,
    ) {
        self.display
            .draw_ex(draw, bkg, self.frame_to_display_coordinates() * trans, z)
    }
    /// Fit (the entire display) to be fully visible
    pub fn fit_display(&mut self, margin: f64) {
        self.display.fit_to_visible_area(margin).nuts_check();
    }
    /// Draw onto the display area from a mesh of triangles. Useful for custom tesselation.
    pub fn draw_mesh(&mut self, mesh: &AbstractMesh) {
        let frame_transform = self.frame_to_display_coordinates();
        self.display.draw_mesh_ex(mesh, frame_transform, 0);
    }
    /// Draw onto the display area from a mesh of triangles. The transformation will be applied to each triangle.
    pub fn draw_mesh_ex(&mut self, mesh: &AbstractMesh, t: Transform, z: i16) {
        let frame_transform = self.frame_to_display_coordinates();
        self.display.draw_mesh_ex(mesh, frame_transform * t, z);
    }
    pub fn add_html(&self, element: Element) {
        if let Some(parent) = self.div.parent_element().nuts_check() {
            parent
                .append_with_node_1(&element)
                .map_err(|e| ErrorMessage::technical(format!("Failed to add HTML: {:?}", e)))
                .nuts_check();
        }
    }
    /// The size of the selected area, in game coordinates
    pub fn size(&self) -> Vector {
        self.region.size()
    }
}

impl Into<DisplayArea> for Display {
    fn into(self) -> DisplayArea {
        DisplayArea {
            region: Rectangle::new_sized(self.game_coordinates),
            div: self.div.clone(),
            display: self,
        }
    }
}
