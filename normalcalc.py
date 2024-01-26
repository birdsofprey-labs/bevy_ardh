import taichi as ti

@ti.func
def trace(heightMap, px, output):
    radius = 0.51
    maxIterations = 1
    x = float(px[0])
    y = float(px[1])
   
    radius2 = 2.0 * radius

    for i in range(maxIterations):
        ox = radius #(ti.random() * 2 - 1) * radius # The X offset
        oy = radius #(ti.random() * 2 - 1) * radius # The Y offset
        # Get the surface normal of the terrain at the current location
        pleft = bilerp(heightMap, ti.Vector([(x - ox), y]))
        ptop = bilerp(heightMap, ti.Vector([x, (y + oy)]))
        pright = bilerp(heightMap, ti.Vector([x + ox, y]))
        pbottom = bilerp(heightMap, ti.Vector([x, (y - oy)]))
        surfaceNormal = ti.Vector([
            radius2 * (pright - pleft),
            0.001,
            radius2 * (- pbottom + ptop)
        ])

        surfaceNormal = ti.math.normalize(surfaceNormal)
        
        # Calculate the deposition and erosion rate
        z_xi = ti.floor(x)
        z_yi = ti.floor(y)

        fx = x - z_xi
        fy = y - z_yi

        delta =  surfaceNormal / maxIterations #* 100.000

        for c in range(3):
            output[int(z_xi), int(z_yi) , c] += fx * fy * delta[c]
            output[int(z_xi + 1), int(z_yi), c] += (1 - fx) * fy * delta[c]
            output[int(z_xi), int(z_yi + 1), c] += fx * (1 - fy) * delta[c]
            output[int(z_xi + 1), int(z_yi + 1), c] += (1 - fx) * (1 - fy) * delta[c]

 
D = ti.Vector([1, 0, -1])
@ti.func
def bilerp(field: ti.template(), P):
    '''
    Bilinear sampling an 2D field with a real index.

    :parameter field: (2D Tensor)
        Specify the field to sample.

    :parameter P: (2D Vector of float)
        Specify the index in field.

    :note:
        If one of the element to be accessed is out of `field.shape`, then
        `bilerp` will automatically do a clamp for you, see :func:`sample`. 

    :return:
        The return value is calcuated as::

            I = int(P)
            x = fract(P)
            y = 1 - x
            return (sample(field, I + D.xx) * x.x * x.y +
                    sample(field, I + D.xy) * x.x * y.y +
                    sample(field, I + D.yy) * y.x * y.y +
                    sample(field, I + D.yx) * y.x * x.y)

        .. where D = vec(1, 0, -1)
    '''
    I = int(P)
    x = ti.math.fract(P)
    y = 1 - x
    return (sample(field, I + D.xx) * x.x * x.y +
            sample(field, I + D.xy) * x.x * y.y +
            sample(field, I + D.yy) * y.x * y.y +
            sample(field, I + D.yx) * y.x * x.y)


@ti.func
def sample(field: ti.template(), P):
    '''
    Sampling a field with indices clampped into the field shape.

    :parameter field: (Tensor)
        Specify the field to sample.

    :parameter P: (Vector)
        Specify the index in field.

    :return:
        The return value is calcuated as::

            P = clamp(P, 0, vec(*field.shape) - 1)
            return field[int(P)]
    '''
    shape = ti.Vector(field.shape)
    #P = ti.math.clamp(P, 0, shape - 1)
    return field[ti.Vector([P.x, P.y, 0])]

@ti.func
def computeSphereGradient(spherePos, normal):
    #phi = ti.math.atan(spherePos.z, spherePos.x)
    #theta = ti.math.acos(spherePos.y)

    dphi = ti.Vector([-spherePos.z, 0.0, spherePos.x]) / (spherePos.x * spherePos.x + spherePos.z * spherePos.z)
    dtheta = ti.Vector([spherePos.x, spherePos.y, spherePos.z]) / ti.math.sqrt(spherePos.x * spherePos.x + spherePos.y * spherePos.y + spherePos.z * spherePos.z)

    dX = ti.math.cross(dphi, normal)
    dY = ti.math.cross(normal, dtheta)

    du = ti.math.dot(dX, normal)
    dv = ti.math.dot(dY, normal)

    return ti.Vector([du, dv])


# @ti.func
# def getNormal(p):
#     eps = 0.0001
#     h = vec2(eps,0)
#     return ti.math.normalize( ti.math.Vecor([ g(p-h.xy) - g(p+h.xy),
#                             2.0*h.x,
#                             g(p-h.yx) - g(p+h.yx)] ) );


#@ti.func
def calcNormal(p, f): # for function f(p)
    h = 0.00001 #.025 #0.001 # replace by an appropriate value
    x = 1.0
    y = -1.0
    xyy = ti.Vector([x,y,y])
    yyx = ti.Vector([y,y,x])
    yxy = ti.Vector([y,x,y])
    xxx = ti.Vector([x,x,x])
    return ti.math.normalize( xyy*f( p + xyy*h ) + 
                      yyx*f( p + yyx*h ) + 
                      yxy*f( p + yxy*h ) + 
                      xxx*f( p + xxx*h ) )
