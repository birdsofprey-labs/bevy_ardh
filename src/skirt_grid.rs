use bevy::{
    pbr::wireframe::{NoWireframe, Wireframe, WireframeColor, WireframeConfig, WireframePlugin},
    prelude::*,
    render::{
        render_resource::WgpuFeatures,
        settings::{RenderCreation, WgpuSettings},
        RenderPlugin,
    },
};
//use bevy_math::*;
//use bevy::render::mesh::shape::{Mesh};

/// A square on the `XZ` plane centered at the origin.
#[derive(Debug, Copy, Clone)]
pub struct GridWithSkirts {
    /// The total side length of the square.
    pub size: f32,
    /// The number of subdivisions in the mesh.
    ///
    /// 0 - is the original plane geometry, the 4 points in the XZ plane.
    ///
    /// 1 - is split by 1 line in the middle of the plane on both the X axis and the Z axis, resulting in a plane with 4 quads / 8 triangles.
    ///
    /// 2 - is a plane split by 2 lines on both the X and Z axes, subdividing the plane into 3 equal sections along each axis, resulting in a plane with 9 quads / 18 triangles.
    ///
    /// and so on...
    pub subdivisions: u32,
    /// skirt length
    pub length: f32,
    /// skirt offscale,
    pub offscale: f32
}


impl Default for GridWithSkirts {
    fn default() -> Self {
        GridWithSkirts {
            size: 1.0,
            length: -1.0,
            offscale: 1.0,
            subdivisions: 0,
        }
    }
}

impl GridWithSkirts {
    /// Creates a new plane centered at the origin with the supplied side length and zero subdivisions.
    pub fn from_size(size: f32) -> Self {
        Self {
            size,
            subdivisions: 16,
            offscale: 0.98,
            length: -0.5
        }
    }
}

impl From<GridWithSkirts> for Mesh {
    fn from(plane: GridWithSkirts) -> Self {
        // here this is split in the z and x directions if one ever needs asymmetrical subdivision
        // two Plane struct fields would need to be added instead of the single subdivisions field
        let z_vertex_count = plane.subdivisions + 2;
        let x_vertex_count = plane.subdivisions + 2;
        let num_vertices = (z_vertex_count * x_vertex_count) as usize;
        let num_indices = ((z_vertex_count - 1) * (x_vertex_count - 1) * 6) as usize;
        //let mut up = Vec3::Y.to_array();

        let mut positions: Vec<[f32; 3]> = Vec::with_capacity(num_vertices);
        let mut normals: Vec<[f32; 3]> = Vec::with_capacity(num_vertices);
        let mut uvs: Vec<[f32; 2]> = Vec::with_capacity(num_vertices);
        let mut indices: Vec<u32> = Vec::with_capacity(num_indices);


        let z_vertex_count = z_vertex_count + 2;
        let x_vertex_count = x_vertex_count + 2;

        let oa = (z_vertex_count-1) as f32 / (z_vertex_count - 3) as f32;
        //let ob = 1f32;//(z_vertex_count-1) as f32 / (z_vertex_count - 1) as f32;
        let dt = oa * plane.size - plane.size;
        let dr = 1.0 / (plane.subdivisions+3) as f32;
        let ds = (1.0+dr) / (((plane.subdivisions+2) as f32) / (plane.subdivisions+3) as f32);
        for z in 0..z_vertex_count {
            //let y = 0.0;
            //if z == 0 { z = 1; iy = 1.0; }
            //if z >= z_vertex_count-1 { z = z_vertex_count-2; y = 1.0; }
            for x in 0..x_vertex_count {

                let mut up = Vec3::Y.to_array();
                let mut x2 = x;
                let mut z2 = z;
                let mut y = 0.0;
                //let mut y = 0.0;
                let psize = plane.size + dt;// *  (2 as f32 / z_vertex_count as f32 );
                let mut oscalex = 1.0;
                let mut oscalez = 1.0;
                if x >= x_vertex_count-1 { x2 = x_vertex_count-2; y = 0.0; oscalex = plane.offscale; up[1] = -1.0; }
                if z >= z_vertex_count-1 { z2 = z_vertex_count-2; y = 0.0; oscalez = plane.offscale; up[1] = -1.0; }
                if x == 0 { x2 = 1; y = 0.0; oscalex = plane.offscale; up[1] = -1.0; }
                if z == 0 { z2 = 1; y = 0.0; oscalez = plane.offscale; up[1] = -1.0; }
                let tx = x2 as f32 / (x_vertex_count - 1) as f32;
                let tz = z2 as f32 / (z_vertex_count - 1) as f32;
                let ux = x as f32 / (x_vertex_count - 1) as f32;
                let uz = z as f32 / (z_vertex_count - 1) as f32;
                positions.push([(-0.5 + tx) * psize * oscalex, y, (-0.5 + tz) * psize * oscalez]);
                normals.push(up);
                //let mf = 1.0 + 1.0 / 130.0;// + dt / plane.size;
                println!("{} x {}  {:?} ", x, z, [(ux-dr)*ds, (uz-dr)*ds]);
                uvs.push([(ux-dr)*ds, (uz-dr)*ds]);
            }
        }

        for y in 0..z_vertex_count - 1 {
            for x in 0..x_vertex_count - 1  {
                let quad = y * x_vertex_count + x;
                indices.push(quad + x_vertex_count + 1 );
                indices.push(quad + 1 );
                indices.push(quad + x_vertex_count);
                indices.push(quad);
                indices.push(quad + x_vertex_count);
                indices.push(quad + 1);
            }
        }

        Mesh::new(
            bevy::render::mesh::PrimitiveTopology::TriangleList)
        .with_indices(Some(bevy::render::mesh::Indices::U32(indices)))
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, positions)
        .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, normals)
        .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, uvs)
    }
}
