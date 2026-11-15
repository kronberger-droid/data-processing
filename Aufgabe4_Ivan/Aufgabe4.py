import random
import math
import os

os.makedirs("Aufgabe4_Ivan", exist_ok=True)


# 1
class Zufallsgenerator:
    def __init__(self, a, c, m, y1):
        self.a = a
        self.c = c
        self.m = m
        self.y = y1

    def rand(self):
        self.y = (self.a * self.y + self.c) % self.m
        return self.y

    def uni01(self):
        return self.rand() / self.m

    def uni(self, imax):
        return self.rand() * (imax + 1) // self.m


for name, a, c, m in [("C", 1103515245, 12345, 2**31),
                       ("RANDU", 65539, 0, 2**31)]:
    z = Zufallsgenerator(a, c, m, 1)

    with open("Aufgabe4_Ivan/" + name + ".dat", "w") as f:
        for i in range(5000):
            f.write(f"{z.uni01()} {z.uni01()} {z.uni01()}\n")


# 2a
def integral(f, a, b, N):
    summe = 0
    for i in range(N):
        summe += f(random.uniform(a, b))
    return (b - a) * summe / N


with open("Aufgabe4_Ivan/integral.dat", "w") as f:
    for N in [10, 100, 1000, 10000, 100000]:
        ergebnis = integral(lambda x: x**2, 0, 1, N)
        f.write(f"{N} {abs(ergebnis - 1/3)}\n")
        print(N, ergebnis)


# 2b
def kugel(n, N):
    treffer = 0

    for i in range(N):
        summe = 0
        for j in range(n):
            x = random.uniform(-1, 1)
            summe += x**2

        if summe <= 1:
            treffer += 1

    return 2**n * treffer / N


with open("Aufgabe4_Ivan/kugel.dat", "w") as f:
    for n in range(1, 13):
        volumen = kugel(n, 200000)
        exakt = math.pi**(n/2) / math.gamma(n/2 + 1)

        f.write(f"{n} {volumen / 2**n} {exakt / 2**n}\n")
        print(n, volumen, exakt)


# Gnuplot
with open("Aufgabe4_Ivan/plots.gp", "w") as f:
    f.write("""
set terminal pngcairo

set output 'Aufgabe4_Ivan/C.png'
splot 'Aufgabe4_Ivan/C.dat' using 1:2:3 with points

set output 'Aufgabe4_Ivan/RANDU.png'
splot 'Aufgabe4_Ivan/RANDU.dat' using 1:2:3 with points

set output 'Aufgabe4_Ivan/integral.png'
set logscale x
plot 'Aufgabe4_Ivan/integral.dat' using 1:2 with linespoints

unset logscale x
set output 'Aufgabe4_Ivan/kugel.png'
plot 'Aufgabe4_Ivan/kugel.dat' using 1:2 with linespoints title 'Monte Carlo', \
     'Aufgabe4_Ivan/kugel.dat' using 1:3 with linespoints title 'Analytisch'

unset output
""")

print("Fertig. Alle Dateien liegen in Aufgabe4_Ivan.")
print("Diagramme erstellen: gnuplot Aufgabe4_Ivan/plots.gp")