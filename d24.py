from sympy import *

x0 = 219051609191782
y0 = 68260434807407
z0 = 317809635461867
dx0 = 146
dy0 = 364
dz0 = -22
x1 = 292151991892724
y1 = 394725036264709
z1 = 272229701860796
dx1 = -43
dy1 = -280
dz1 = -32
x2 = 455400538938496
y2 = 167482380286201
z2 = 389150487664328
dx2 = -109
dy2 = 219
dz2 = -58

t0, t1 = symbols('t0,t1')
p0x = x0 + t0 * dx0
p0y = y0 + t0 * dy0
p0z = z0 + t0 * dz0
p1x = x1 + t1 * dx1
p1y = y1 + t1 * dy1
p1z = z1 + t1 * dz1
vx = (p1x - p0x) / (t1 - t0)
vy = (p1y - p0y) / (t1 - t0)
vz = (p1z - p0z) / (t1 - t0)
tx = (x2 + t0 * vx - p0x) / (vx - dx2)
ty = (y2 + t0 * vy - p0y) / (vy - dy2)
tz = (z2 + t0 * vz - p0z) / (vz - dz2)
sol_t1 = solve((tx - ty), t1)[0]
print(f"sol_t1 = {sol_t1}")
print(f"subs = {simplify((tx - tz).subs(t1, sol_t1))}")
sol_t0 = solve((tx - tz).subs(t1, sol_t1))[0]
print(f"sol_t0 = {sol_t0}")
num_t0 = int(sol_t0)
num_t1 = int(sol_t1.subs(t0, num_t0))
x = x0 + t0 * (dx0 - vx)
y = y0 + t0 * (dy0 - vy)
z = z0 + t0 * (dz0 - vz)
num_x = x.subs(t0, num_t0).subs(t1, num_t1)
num_y = y.subs(t0, num_t0).subs(t1, num_t1)
num_z = z.subs(t0, num_t0).subs(t1, num_t1)
print(num_x+num_y+num_z)
