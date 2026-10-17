
import math

R = 6371000 


punkte = []

with open("Aufgabe1_Ivan/daten/Gpsies_austria.csv", "r") as datei:
    for zeile in datei:
        if zeile.startswith("#") or not zeile.strip():
            continue

        breite, laenge, hoehe, nummer = map(float, zeile.split())
        punkte.append([math.radians(breite), math.radians(laenge)])

lat0 = sum(p[0] for p in punkte) / len(punkte)
lon0 = sum(p[1] for p in punkte) / len(punkte)


xy = []

for lat, lon in punkte:
    x = R * (lon - lon0) * math.cos(lat0)
    y = R * (lat - lat0)
    xy.append([x, y])

umfang = 0
flaeche = 0
sx = 0
sy = 0

for i in range(len(punkte)):
    j = (i + 1) % len(punkte)

    lat1, lon1 = punkte[i]
    lat2, lon2 = punkte[j]

    a = (math.sin((lat2 - lat1) / 2)**2
         + math.cos(lat1) * math.cos(lat2)
         * math.sin((lon2 - lon1) / 2)**2)

    umfang += 2 * R * math.asin(math.sqrt(min(1, a)))


    x1, y1 = xy[i]
    x2, y2 = xy[j]

    kreuz = x1 * y2 - x2 * y1

    flaeche += kreuz
    sx += (x1 + x2) * kreuz
    sy += (y1 + y2) * kreuz


sx = sx / (3 * flaeche)
sy = sy / (3 * flaeche)

schwerpunkt_lat = lat0 + sy / R
schwerpunkt_lon = lon0 + sx / (R * math.cos(lat0))

flaeche = abs(flaeche) / 2

print("Umfang:", umfang / 1000, "km")
print("Fläche:", flaeche / 1000000, "km²")
print("Schwerpunkt:")
print("Breite:", math.degrees(schwerpunkt_lat))
print("Länge:", math.degrees(schwerpunkt_lon))


import matplotlib.pyplot as plt

breiten = [math.degrees(p[0]) for p in punkte]
laengen = [math.degrees(p[1]) for p in punkte]

plt.figure(figsize=(12, 7))

plt.plot(laengen + [laengen[0]],
         breiten + [breiten[0]],
         "b-", linewidth=1)

plt.fill(laengen, breiten, color="lightblue", alpha=0.4)

plt.scatter(
    math.degrees(schwerpunkt_lon),
    math.degrees(schwerpunkt_lat),
    color="red",
    s=100,
    label="Schwerpunkt",
    zorder=5
)

plt.xlabel("Längengrad")
plt.ylabel("Breitengrad")
plt.title("Österreich – GPS-Polygon und Schwerpunkt")
plt.legend()
plt.grid(True)
plt.axis("equal")

plt.show()