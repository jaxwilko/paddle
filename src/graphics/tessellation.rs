//! Tessellation is the act of transforming (complex) geometric objects to a mesh of triangles.
use crate::{
    quicksilver_compat::{
        geom::{Line, Triangle},
        Background, Circle, Shape,
    },
    Rectangle, Transform, Vector,
};

mod abstract_mesh;
mod lyon_tessellator;

pub use abstract_mesh::*;
pub use lyon_tessellator::*;

/// Some geometry object that can be tessellated to an AbstractMesh of triangles
pub trait Tessellate {
    fn tessellate<'a>(&self, mesh: &mut AbstractMesh, background: Background<'a>);
}

impl Tessellate for Vector {
    fn tessellate<'a>(&self, mesh: &mut AbstractMesh, bkg: Background<'a>) {
        Rectangle::new(*self, Vector::ONE).tessellate(mesh, bkg);
    }
}

impl Tessellate for Rectangle {
    fn tessellate<'a>(&self, mesh: &mut AbstractMesh, bkg: Background<'a>) {
        let trans = Transform::translate(self.top_left() + self.size() / 2)
            * Transform::translate(-self.size() / 2)
            * Transform::scale(self.size());
        let tex_trans = bkg.texture_transform();
        let offset = mesh.add_positioned_vertices(
            [Vector::ZERO, Vector::X, Vector::ONE, Vector::Y]
                .iter()
                .cloned(),
            trans,
            tex_trans,
            bkg,
        );
        mesh.triangles
            .push(AbstractTriangle::new(offset, [0, 1, 2], bkg));
        mesh.triangles
            .push(AbstractTriangle::new(offset, [2, 3, 0], bkg));
    }
}

impl Tessellate for Circle {
    fn tessellate<'a>(&self, mesh: &mut AbstractMesh, bkg: Background<'a>) {
        let trans =
            Transform::translate(self.center()) * Transform::scale(Vector::ONE * self.radius);
        let tex_trans = bkg.texture_transform();
        let offset =
            mesh.add_positioned_vertices(CIRCLE_POINTS.iter().cloned(), trans, tex_trans, bkg);
        mesh.triangles.extend(
            (0..CIRCLE_POINTS.len()).map(|index| {
                AbstractTriangle::new(offset, [0, index as u32, index as u32 + 1], bkg)
            }),
        );
    }
}

impl Tessellate for Triangle {
    fn tessellate<'a>(&self, mesh: &mut AbstractMesh, bkg: Background<'a>) {
        let trans = Transform::translate(self.center()) * Transform::translate(-self.center());
        let tex_trans = bkg.texture_transform();
        let offset = mesh.add_positioned_vertices(
            [self.a, self.b, self.c].iter().cloned(),
            trans,
            tex_trans,
            bkg,
        );
        mesh.triangles
            .push(AbstractTriangle::new(offset, [0, 1, 2], bkg));
    }
}

impl Tessellate for Line {
    fn tessellate<'a>(&self, mesh: &mut AbstractMesh, bkg: Background<'a>) {
        // create rectangle in right size
        let rect = Rectangle::new(
            (self.a.x, self.a.y + self.t / 2.0),
            (self.a.distance(self.b), self.t),
        );

        rect.tessellate(mesh, bkg)
    }
}

// Taken from quicksilver:
// Until there's serious compile-time calculations in Rust,
// it's best to just pre-write the points on a rasterized circle

// Python script to generate the array:
/*
import math
from math import pi


def points_on_circumference(center=(0, 0), r=50, n=100):
    n -= 1
    return [
        (
            center[0]+(math.cos(2 * pi / n * x) * r),  # x
            center[1] + (math.sin(2 * pi / n * x) * r)  # y

        ) for x in range(0, n + 1)]


number_of_points = 64
points = points_on_circumference(center=(0,0),r=1, n=number_of_points)

print("const CIRCLE_POINTS: [Vector; %i] = [" % number_of_points)

for point in points:
    print("    Vector { x: %s, y: %s }," % (point[0], point[1]))

print("];")
*/

const CIRCLE_POINTS: [Vector; 64] = [
    Vector { x: 1.0, y: 0.0 },
    Vector {
        x: 0.9950307753654014,
        y: 0.09956784659581666,
    },
    Vector {
        x: 0.9801724878485438,
        y: 0.19814614319939758,
    },
    Vector {
        x: 0.9555728057861407,
        y: 0.2947551744109042,
    },
    Vector {
        x: 0.9214762118704076,
        y: 0.38843479627469474,
    },
    Vector {
        x: 0.8782215733702285,
        y: 0.47825397862131824,
    },
    Vector {
        x: 0.8262387743159949,
        y: 0.5633200580636221,
    },
    Vector {
        x: 0.766044443118978,
        y: 0.6427876096865394,
    },
    Vector {
        x: 0.6982368180860729,
        y: 0.7158668492597184,
    },
    Vector {
        x: 0.6234898018587336,
        y: 0.7818314824680298,
    },
    Vector {
        x: 0.5425462638657594,
        y: 0.8400259231507714,
    },
    Vector {
        x: 0.4562106573531629,
        y: 0.8898718088114687,
    },
    Vector {
        x: 0.365341024366395,
        y: 0.9308737486442042,
    },
    Vector {
        x: 0.27084046814300516,
        y: 0.962624246950012,
    },
    Vector {
        x: 0.17364817766693022,
        y: 0.9848077530122081,
    },
    Vector {
        x: 0.07473009358642417,
        y: 0.9972037971811801,
    },
    Vector {
        x: -0.024930691738072913,
        y: 0.9996891820008162,
    },
    Vector {
        x: -0.12434370464748516,
        y: 0.9922392066001721,
    },
    Vector {
        x: -0.22252093395631434,
        y: 0.9749279121818236,
    },
    Vector {
        x: -0.31848665025168454,
        y: 0.9479273461671317,
    },
    Vector {
        x: -0.41128710313061156,
        y: 0.9115058523116731,
    },
    Vector {
        x: -0.5000000000000002,
        y: 0.8660254037844385,
    },
    Vector {
        x: -0.58374367223479,
        y: 0.8119380057158564,
    },
    Vector {
        x: -0.6616858375968595,
        y: 0.7497812029677341,
    },
    Vector {
        x: -0.7330518718298263,
        y: 0.6801727377709194,
    },
    Vector {
        x: -0.7971325072229225,
        y: 0.6038044103254774,
    },
    Vector {
        x: -0.8532908816321556,
        y: 0.5214352033794981,
    },
    Vector {
        x: -0.900968867902419,
        y: 0.43388373911755823,
    },
    Vector {
        x: -0.9396926207859084,
        y: 0.3420201433256685,
    },
    Vector {
        x: -0.969077286229078,
        y: 0.24675739769029342,
    },
    Vector {
        x: -0.9888308262251285,
        y: 0.14904226617617428,
    },
    Vector {
        x: -0.9987569212189223,
        y: 0.04984588566069704,
    },
    Vector {
        x: -0.9987569212189223,
        y: -0.04984588566069723,
    },
    Vector {
        x: -0.9888308262251285,
        y: -0.14904226617617447,
    },
    Vector {
        x: -0.969077286229078,
        y: -0.24675739769029362,
    },
    Vector {
        x: -0.9396926207859084,
        y: -0.34202014332566866,
    },
    Vector {
        x: -0.9009688679024191,
        y: -0.433883739117558,
    },
    Vector {
        x: -0.8532908816321555,
        y: -0.5214352033794983,
    },
    Vector {
        x: -0.7971325072229224,
        y: -0.6038044103254775,
    },
    Vector {
        x: -0.7330518718298262,
        y: -0.6801727377709195,
    },
    Vector {
        x: -0.6616858375968594,
        y: -0.7497812029677342,
    },
    Vector {
        x: -0.5837436722347898,
        y: -0.8119380057158565,
    },
    Vector {
        x: -0.4999999999999996,
        y: -0.8660254037844388,
    },
    Vector {
        x: -0.4112871031306116,
        y: -0.9115058523116731,
    },
    Vector {
        x: -0.3184866502516841,
        y: -0.9479273461671318,
    },
    Vector {
        x: -0.2225209339563146,
        y: -0.9749279121818236,
    },
    Vector {
        x: -0.12434370464748495,
        y: -0.9922392066001721,
    },
    Vector {
        x: -0.024930691738073156,
        y: -0.9996891820008162,
    },
    Vector {
        x: 0.07473009358642436,
        y: -0.9972037971811801,
    },
    Vector {
        x: 0.17364817766693083,
        y: -0.984807753012208,
    },
    Vector {
        x: 0.2708404681430051,
        y: -0.962624246950012,
    },
    Vector {
        x: 0.3653410243663954,
        y: -0.9308737486442041,
    },
    Vector {
        x: 0.45621065735316285,
        y: -0.8898718088114687,
    },
    Vector {
        x: 0.5425462638657597,
        y: -0.8400259231507713,
    },
    Vector {
        x: 0.6234898018587334,
        y: -0.7818314824680299,
    },
    Vector {
        x: 0.698236818086073,
        y: -0.7158668492597183,
    },
    Vector {
        x: 0.7660444431189785,
        y: -0.6427876096865389,
    },
    Vector {
        x: 0.8262387743159949,
        y: -0.563320058063622,
    },
    Vector {
        x: 0.8782215733702288,
        y: -0.4782539786213178,
    },
    Vector {
        x: 0.9214762118704076,
        y: -0.38843479627469474,
    },
    Vector {
        x: 0.9555728057861408,
        y: -0.2947551744109039,
    },
    Vector {
        x: 0.9801724878485438,
        y: -0.19814614319939772,
    },
    Vector {
        x: 0.9950307753654014,
        y: -0.09956784659581641,
    },
    Vector { x: 1.0, y: 0.0 },
];
