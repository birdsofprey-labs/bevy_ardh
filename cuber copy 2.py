from taichi.examples.patterns import taichi_logo

import taichi as ti

ti.init(arch=ti.vulkan)

k = 1024
N_SIDES = 6

xside_cols = [ti.Vector([1.0, 0.0, 0.0]), ti.Vector([0.0, 1.0, 0.0]), ti.Vector([0.0, 0.0, 1.0]),
            ti.Vector([1.0, 1.0, 0.0]), ti.Vector([0.0, 1.0, 1.0]), ti.Vector([1.0, 0.0, 1.0])]

side_cols = ti.Vector.field(3, dtype=float, shape=N_SIDES)
for i,x in enumerate(xside_cols):
    side_cols[i] = xside_cols[i]
res = (k, k)
pix_sides = ti.Vector.field(3, dtype=float, shape=(N_SIDES, res[0], res[1]))
tex_sides = [ti.Texture(ti.Format.rgba16f, (k, k)) ] * N_SIDES 

@ti.func
def convert_cube_uv_to_xyz(index, uv):
    u = uv.x
    v = uv.y
    uc = 2.0 * u - 1.0
    vc = 2.0 * v - 1.0
    
    x = y = z = 0.0
    
    if index == 0:
        x, y, z = 1.0, vc, -uc  # POSITIVE X
    elif index == 1:
        x, y, z = -1.0, vc, uc  # NEGATIVE X
    elif index == 2:
        x, y, z = uc, 1.0, -vc  # POSITIVE Y
    elif index == 3:
        x, y, z = uc, -1.0, vc  # NEGATIVE Y
    elif index == 4:
        x, y, z = uc, vc, 1.0  # POSITIVE Z
    elif index == 5:
        x, y, z = -uc, vc, -1.0  # NEGATIVE Z
    
    return ti.Vector([x, y, z])


@ti.kernel
def make_texture(index: int, tex: ti.types.rw_texture(num_dimensions=2, fmt=ti.Format.rgba16f, lod=0), dims: ti.i32):
    for i, j in ti.ndrange(dims, dims):
        uv = ti.Vector([i/dims, j/dims])
        xyz = convert_cube_uv_to_xyz(index, uv)
        vbase = ti.math.normalize(xyz)
        packed_normals = (vbase + ti.Vector([1.0, 1.0, 1.0])) / 2.0
        ret =  ti.math.normalize(packed_normals)       #side_cols[ti.cast(index, ti.i32)] #ti.cast(taichi_logo(uv / dims), ti.f32)
        #tex.store(ti.Vector([i, j]), ti.Vector([ret.x, ret.y, ret.z, 1.0]))
        pix_sides[index, i, j] = [ret.r*1, ret.g*1, ret.b*1]


@ti.kernel
def paint(index: ti.u32, tex: ti.types.texture(num_dimensions=2), n: ti.i32):
    for i, j in ti.ndrange(n, n):
        uv = ti.Vector([i / res[0], j / res[1]])
        c = ti.math.vec4(0.0)
        c = tex.sample_lod(uv, 0.0)
        pix_sides[index, i, j] = [c.r*1, c.g*1, c.b*1]
        pass


def main():
    #window = ti.ui.Window("UV", res)
    #canvas = window.get_canvas()

    t = 0.0
    for i in range(N_SIDES):
        make_texture(i, tex_sides[i], k)
        #paint(i, tex_sides[i], k)
    #canvas.set_image(pixels)
    #window.show()
        ti.tools.image.imwrite(pix_sides.to_numpy()[i], f'gallery/test0{i}.png')#

if __name__ == "__main__":
    main()
