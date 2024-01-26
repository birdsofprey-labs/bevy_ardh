from taichi.examples.patterns import taichi_logo

import taichi as ti
import noiselib, normalcalc
ti.init(arch=ti.cpu)

k = 1024*12
N_SIDES = 6

xside_cols = [ti.Vector([1.0, 0.0, 0.0]), ti.Vector([0.0, 1.0, 0.0]), ti.Vector([0.0, 0.0, 1.0]),
            ti.Vector([1.0, 1.0, 0.0]), ti.Vector([0.0, 1.0, 1.0]), ti.Vector([1.0, 0.0, 1.0])]

side_cols = ti.Vector.field(3, dtype=float, shape=N_SIDES)
for i,x in enumerate(xside_cols):
    side_cols[i] = xside_cols[i]
res = (k, k)

hgt_sides = ti.Vector.field(1, dtype=float, shape=(N_SIDES, res[0], res[1]))
nor_sides = ti.Vector.field(3, dtype=float, shape=(N_SIDES, res[0], res[1]))
#tex_sides = [ti.Texture(ti.Format.rgba16f, (k, k)) ] * N_SIDES 

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

heightfn2 = lambda p: noiselib.fbm(p * 18.0, 18)
heightfn = lambda p: heightfn2(p) * heightfn2(p)

@ti.kernel
def make_height_map(index: int, dims: ti.i32):
    for i, j in ti.ndrange(dims, dims):
        uv = ti.Vector([i/dims, j/dims])
        xyz = convert_cube_uv_to_xyz(index, uv)
        norm = ti.math.normalize(xyz) #* 10.0
        #height = noiselib.fbm(norm, 16)
        height = heightfn(norm)
        packed_height = height#(height + 1.0) / 2.0
        ret = packed_height       #side_cols[ti.cast(index, ti.i32)] #ti.cast(taichi_logo(uv / dims), ti.f32)
        #tex.store(ti.Vector([i, j]), ti.Vector([ret.x, ret.y, ret.z, 1.0]))
        hgt_sides[index, i, j] = ret

@ti.kernel
def make_normals(index: int, dims: ti.i32):
    for i, j in ti.ndrange(dims, dims):
        uv = ti.Vector([i/dims, j/dims])
        xyz = convert_cube_uv_to_xyz(index, uv)

        norm = ti.math.normalize(xyz)

        N = norm - 0.5*normalcalc.calcNormal(norm, heightfn)
        N = ti.math.normalize(N)

        #N = ti.math.normalize(xyz)
        T = ti.math.normalize(ti.math.cross(ti.Vector([0.0, 1.0, 0.0]), N))
        B = ti.math.cross(N, T)
        #dN = normalcalc.computeSphereGradient(N,N) * 10000000.0
        packed_normals = (N + ti.Vector([1.0, 1.0, 1.0])) / 2.0
        ret =  (packed_normals)       #side_cols[ti.cast(index, ti.i32)] #ti.cast(taichi_logo(uv / dims), ti.f32)
        #tex.store(ti.Vector([i, j]), ti.Vector([ret.x, ret.y, ret.z, 1.0]))
        nor_sides[index, i, j] = [ret.x, ret.y, ret.z]


@ti.kernel
def paint(index: ti.u32, tex: ti.types.texture(num_dimensions=2), n: ti.i32):
    for i, j in ti.ndrange(n, n):
        uv = ti.Vector([i / res[0], j / res[1]])
        c = ti.math.vec4(0.0)
        c = tex.sample_lod(uv, 0.0)
        make_height_map[index, i, j] = [c.r*1, c.g*1, c.b*1]
        pass


def main():
    #window = ti.ui.Window("UV", res)
    #canvas = window.get_canvas()

    t = 0.0
    for i in range(N_SIDES):
        make_height_map(i, k)
        make_normals(i, k)
        #paint(i, tex_sides[i], k)
    #canvas.set_image(pixels)
    #window.show()
        ti.tools.image.imwrite(hgt_sides.to_numpy()[i], f'gallery/hgt{i}.png')#
        ti.tools.image.imwrite(nor_sides.to_numpy()[i], f'gallery/nor{i}.png')#

if __name__ == "__main__":
    main()
