

from math import pi, exp
import matplotlib.pyplot as plt

G = 6.67430e-11
M_ERDE = 5.9722e24
R_ERDE = 6371000

MASSE = 0.007874
VOLUMEN = 1e-6

RADIUS = (3 * VOLUMEN / (4 * pi)) ** (1 / 3)
A = pi * RADIUS**2

CW = 0.47
RHO0 = 1.225
M_LUFT = 0.02896
R_GAS = 8.314
T = 288.15

DT = 0.0001


def beschleunigung(h, v):
    g = G * M_ERDE / (R_ERDE + h)**2

    rho = RHO0 * exp(
        -M_LUFT * 9.81 * h / (R_GAS * T)
    )

    widerstand = 0.5 * CW * A * rho * v * abs(v)

    return g - widerstand / MASSE


def simulieren(v0):
    t = 0.0
    h = 150000.0
    v = v0

    zeiten = []
    hohen = []
    geschwindigkeiten = []
    beschleunigungen = []

    schritt = 0

    while h > 0 and t < 300:
        a = beschleunigung(h, v)

        if schritt % 1000 == 0:
            zeiten.append(t)
            hohen.append(h)
            geschwindigkeiten.append(v)
            beschleunigungen.append(a)

        h -= v * DT
        v += a * DT
        t += DT

        schritt += 1

    zeiten.append(t)
    hohen.append(max(0, h))
    geschwindigkeiten.append(v)
    beschleunigungen.append(beschleunigung(max(0, h), v))

    return zeiten, hohen, geschwindigkeiten, beschleunigungen, t, h, v


def main():
    fig, achsen = plt.subplots(3, 1, figsize=(9, 11), sharex=True)

    for start in (15000, 25000, 35000):
        ts, hs, vs, acc, ende, h_ende, v_ende = simulieren(start)

        label = f"{start / 1000:.0f} km/s"

        achsen[0].plot(ts, [h / 1000 for h in hs], label=label)
        achsen[1].plot(ts, [v / 1000 for v in vs], label=label)
        achsen[2].plot(ts, acc, label=label)

        if h_ende <= 0:
            print(
                f"Start {label}: Bodenkontakt nach {ende:.2f} s, "
                f"Geschwindigkeit {v_ende:.2f} m/s"
            )
        else:
            print(
                f"Start {label}: Simulation nach {ende:.2f} s beendet, "
                f"Höhe {h_ende:.2f} m, "
                f"Geschwindigkeit {v_ende:.2f} m/s"
            )

    for achse, ylabel in zip(
        achsen,
        ("Höhe [km]", "v [km/s]", "a [m/s²]")
    ):
        achse.set_ylabel(ylabel)
        achse.grid(alpha=0.3)
        achse.legend()

    achsen[-1].set_xlabel("Zeit [s]")

    fig.tight_layout()
    fig.savefig("meteorit.png", dpi=160)

    plt.show()


if __name__ == "__main__":
    main()
