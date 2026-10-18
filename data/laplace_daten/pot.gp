set pm3d
set palette
unset surface
set pm3d at s scansforward
splot 'potential.tmp'
pause -1

# E-Feld
sf=2000.
e(x,y)=x/sqrt(x**2+y**2)
lf=10.
xyf=1000.
plot 'gradient.tmp' u ($1*xyf):($2*xyf):($3/sf):($4/sf) w vec
pause -1
plot 'gradient.tmp' u ($1*xyf):($2*xyf):(e($3,$4)/lf):(e($4,$3)/lf) w vec
pause -1
splot 'g1.tmp' u ($1*xyf):($2*xyf):($1*0-10):(e($3,$4)/lf):(e($4,$3)/lf):($1*0) w vec
pause -1
splot 'potential.tmp','g1.tmp' u ($1*xyf):($2*xyf):($1*0-10):($3/sf):($4/sf):($1*0) w vec
pause -1
#
#
#
set multiplot
set pm3d
set palette
unset surface
set pm3d at s scansforward
splot 'potential.tmp'
reset
set zrange [-10:10]
splot 'g1.tmp' u ($1*xyf):($2*xyf):($1*0-10):(e($3,$4)/lf):(e($4,$3)/lf):($1*0) w vec
unset multiplot
