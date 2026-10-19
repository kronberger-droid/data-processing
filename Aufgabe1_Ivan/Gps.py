
import math

R = 6371000 


punkte = []

with open("Aufgabe1_Ivan/daten/Gpsies_austria.csv", "r") as datei:
    for zeile in datei:
        if zeile.startswith("#") or not zeile.strip():
            ''' ignoriert komentarzeilen und zeilen die nach entfernen der space nichts haben'''
            continue

        breite, laenge, hoehe, nummer = map(float, zeile.split())
        '''entfernt space und wandelt die strinks dan in float mit map um'''
        punkte.append([math.radians(breite), math.radians(laenge)])
        '''wir brauchen radiants und nicht grad, weil python mit radiants rechnet'''

lat0 = sum(p[0] for p in punkte) / len(punkte)
lon0 = sum(p[1] for p in punkte) / len(punkte)
''' der mittelpunkt, für die berechnung, nicht der richtige mitelpunkt'''

xy = []

for lat, lon in punkte:
    """mit bild am besten zu verstehen"""
    x = R * (lon - lon0) * math.cos(lat0)
    y = R * (lat - lat0)
    xy.append([x, y])

umfang = 0
flaeche = 0
sx = 0
sy = 0

for i in range(len(punkte)):
    j = (i + 1) % len(punkte) #trick damit beim letzten punkt j der erste punkt wird

    lat1, lon1 = punkte[i]
    lat2, lon2 = punkte[j]

    a = (math.sin((lat2 - lat1) / 2)**2
         + math.cos(lat1) * math.cos(lat2)
         * math.sin((lon2 - lon1) / 2)**2)

    umfang += 2 * R * math.asin(math.sqrt(min(1, a)))


    x1, y1 = xy[i]
    x2, y2 = xy[j]

    minifleche = x1 * y2 - x2 * y1
    """wie bei den integralen die fleche unter den 2 gps punkten bis x achse nehmen und summieren"""
    flaeche += minifleche
    sx += (x1 + x2) * minifleche # das ist unsere summee von S*A
    sy += (y1 + y2) * minifleche


sx = sx / (3 * flaeche) #da die summe aller A also aller miniflächen die fleche ist gehts einfacher so
sy = sy / (3 * flaeche) # die 3 stammt von 1/3*1/2 1/1/2

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
    label="Schwerpunkt"
)

plt.xlabel("Längengrad")
plt.ylabel("Breitengrad")
plt.title("Österreich – GPS-Polygon und Schwerpunkt")
plt.legend()
plt.grid(True)
plt.axis("equal")

plt.show()