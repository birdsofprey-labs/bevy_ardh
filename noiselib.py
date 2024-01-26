import taichi as ti

@ti.func
def mod289(x):
    return x - ti.floor(x * (1.0 / 289.0)) * 289.0

@ti.func
def perm(x):
    return mod289(((x * 34.0) + 1.0) * x)


@ti.func
def noise(p):
    a = ti.floor(p)
    d = p - a
    d = d * d * (3.0 - 2.0 * d)

    b = a.xxyy + ti.Vector([0.0, 1.0, 0.0, 1.0])
    k1 = perm(b.xyxy)
    k2 = perm(k1.xyxy + b.zzww)

    c = k2 + a.zzzz
    k3 = perm(c)
    k4 = perm(c + 1.0)

    o1 = ti.math.fract(k3 * (1.0 / 41.0))
    o2 = ti.math.fract(k4 * (1.0 / 41.0))

    o3 = o2 * d.z + o1 * (1.0 - d.z)
    o4 = o3.yw * d.x + o3.xz * (1.0 - d.x)

    return o4.y * d.y + o4.x * (1.0 - d.y)

@ti.func
def fbm(x, num_octaves):
	v = 0.0
	a = 0.5
	shift = ti.Vector([100.0, 100.0, 100.0])
	for i in range(num_octaves):
		v += a * noise(x)
		x = x * 2.0 + shift
		a *= 0.5
	return v
