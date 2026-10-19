
import math
import matplotlib.pyplot as plt


def f(x):
    return 2.0 - 0.6 * x**2


def ableitung(x):
    return -1.2 * x


def reflektieren(richtung, normale):
    nx, ny = normale
    dx, dy = richtung

    betrag = math.sqrt(nx**2 + ny**2)

    nx = nx / betrag
    ny = ny / betrag

    skalar = dx * nx + dy * ny

    rx = dx - 2 * skalar * nx
    ry = dy - 2 * skalar * ny

    return rx, ry


# Spiegel zeichnen
x = [-1.5 + i * 3 / 399 for i in range(400)]
y = [f(wert) for wert in x]

plt.figure(figsize=(6, 6))
plt.plot(x, y, color="black", linewidth=1.2)

# Einfallende und reflektierte Strahlen
for i in range(21):
    x0 = -1.5 + i * 3 / 20
    y0 = f(x0)

    # Normale und Einfallsrichtung
    normale = (-ableitung(x0), 1.0)
    einfall = (0.0, 1.0)

    # Reflexion berechnen
    rx, ry = reflektieren(einfall, normale)

    # Einfallender Strahl von unten
    plt.plot(
        [x0, x0],
        [0, y0],
        color="magenta",
        linewidth=0.7
    )

    # Reflektierter Strahl
    laenge = 1.25

    plt.plot(
        [x0, x0 + laenge * rx],
        [y0, y0 + laenge * ry],
        color="turquoise",
        linewidth=0.7
    )


# Darstellung
plt.axis("equal")
plt.xlim(-1.7, 1.7)
plt.ylim(-0.5, 2.3)

plt.xlabel("x")
plt.ylabel("y")
plt.title("Reflexion am gekrümmten Spiegel")
plt.grid(False)

plt.savefig("spiegel.png", dpi=160)
plt.show()
