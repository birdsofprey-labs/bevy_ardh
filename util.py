import taichi as ti

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
    P = ti.math.clamp(P, 0, shape - 1)
    #return field[ti.Vector([P.x, P.y, 0])]
    return field[ti.Vector([P.x, P.y])]