---
title: Analytická Geometria – Cvičenia
author: Michal Spano
header-includes: |
    \usepackage{fancyhdr}
    \usepackage{gensymb}
    \usepackage{textcomp}
    \pagestyle{fancy}
    \fancyhead[CO,CE]{Analytická Geometria – Cvičenia}
    \fancyfoot[CO,CE]{Michal Spano, © 2026}
    \fancyfoot[LE,RO]{\thepage}
papersize: a4
fontsize: 12pt
colorlinks: true
geometry: margin=2cm
output: pdf_document
---

# Vektory

## Základné pojmy

**1)**

Záchranárske stredisko sa nachádza v bode $S = (1, 2)$ a nehoda nastala v bode $N = (7, 6)$.

1. Urči vektor $\overrightarrow{SN}$ (smer od strediska k nehode) a vektor $\overrightarrow{NS}$ (spätná trasa).
1. Platí $\overrightarrow{SN} = \overrightarrow{NS}$? Zdôvodni.
1. Vypočítaj vzdialenosť strediska od nehody.

**2)**

Futbalista stojí na pozícii $F = (2, 3)$ a strieľa na bránku do bodu $G = (10, 9)$.

1. Urči vektor $\overrightarrow{FG}$.
1. Vypočítaj vzdialenosť futbalistu od bránky $|\overrightarrow{FG}|$.
1. Brankár stojí v bode $B = (10, 6)$. Urči vektor $\overrightarrow{GB}$ a vysvetli, čo tento vektor v kontexte hry predstavuje.

**3)**

Robotické auto začína v $A = (0, 0)$, ide do $B = (4, 3)$ a potom do $C = (7, 7)$.

1. Urči vektory $\overrightarrow{AB}$ a $\overrightarrow{BC}$.
1. Vypočítaj dĺžky úsekov $AB$ a $BC$.
1. Aký uhol zatočilo auto v bode $B$? Použij vzorec $\cos\alpha = \frac{\overrightarrow{AB} \cdot \overrightarrow{BC}}{|\overrightarrow{AB}||\overrightarrow{BC}|}$.

## Skalárny súčin a kolmosť

**4)**

Záhradník plánuje záhradu s dvoma plotmi. Jeden plot vedie smerom $\vec{u} = (2, 6)$, druhý smerom $\vec{v} = (-3, 1)$.

1. Vypočítaj skalárny súčin $\vec{u} \cdot \vec{v}$.
1. Sú tieto ploty navzájom kolmé? Zdôvodni.
1. Záhradník chce pridať tretí plot kolmý na $\vec{u}$. Navrhni možný smerový vektor tohto plota.

**5)**

Dve diaľnice vychádzajú z toho istého miesta. Prvá smeruje vektorom $\vec{u} = (3, 1)$, druhá vektorom $\vec{v} = (1, -3)$.

1. Vypočítaj skalárny súčin $\vec{u} \cdot \vec{v}$.
1. Pod akým uhlom sa diaľnice rozchádzajú?
1. Dopravný inžinier tvrdí, že diaľnice sú navzájom kolmé. Má pravdu?

**6)**

Loď vyplávala z prístavu $P = (0, 0)$ a zamierila do prístavu $Q = (5, 12)$ (súradnice v km). V rovnakom čase vyplávala iná loď z $P$ smerom $\vec{w} = (-12, 5)$.

1. Vypočítaj vzdialenosť $|PQ|$.
1. Sú trasy lodí kolmé? Zdôvodni skalárnym súčinom.
1. Čo to v praxi znamená pre navigáciu lodí?

## Uhol medzi vektormi

**7)**

Turistický chodník vedie od základne $A = (0, 0)$ cez bod $B = (6, 2)$ do cieľa $C = (9, 8)$.

1. Urči vektory $\overrightarrow{AB}$ a $\overrightarrow{BC}$.
1. Vypočítaj uhol, o ktorý sa chodník otočí v bode $B$.
1. Je zmena smeru väčšia alebo menšia ako $45\degree$?

**8)**

Slnečné lúče dopadajú na zem smerom $\vec{s} = (2, -3)$. Solárny panel je inštalovaný pozdĺž vektora $\vec{p} = (3, 2)$.

1. Vypočítaj uhol $\alpha$ medzi smerom lúčov a panelom.
1. Panel je najefektívnejší, keď je kolmý na lúče ($\alpha = 90\degree$). Je tento panel optimálne natočený?
1. Aký by musel byť smerový vektor panela, aby bol kolmý na $\vec{s}$?

# Priamky

## Rovnica priamky, smerový a normálový vektor

**9)**

Cesta na mape prechádza bodmi $P = (0, 4)$ a $Q = (6, 0)$.

1. Urči smerový vektor $\overrightarrow{PQ}$ a normálový vektor $\vec{n}$ tejto cesty.
1. Napíš rovnicu cesty v tvare $ax + by = c$.
1. Overí, či bod $R = (3, 2)$ leží na ceste. Čo to geometricky znamená?

**10)**

Dvaja inžinieri navrhujú chodník ($\ell_1: 2x + y = 5$) a cestu ($\ell_2: x - 2y = 3$).

1. Urči normálové vektory $\vec{n_1}$ a $\vec{n_2}$ oboch priamok.
1. Urči smerové vektory $\vec{d_1}$ a $\vec{d_2}$ oboch priamok.
1. Sú chodník a cesta navzájom kolmé? Zdôvodni pomocou skalárneho súčinu smerových vektorov.

**11)**

Priamka $\ell$ prechádza bodmi $M = (1, -1)$ a $N = (5, 3)$.

1. Urči smerový vektor $\overrightarrow{MN}$ a normálový vektor priamky $\ell$.
1. Napíš rovnicu priamky $\ell$ v tvare $ax + by = c$.
1. Bod $K = (3, 1)$ leží na priamke $\ell$? Overí dosadením.

# Kosínusová veta

**12)**

Výprava troch horolezeckých táborov tvorí trojuholník. Tábory $A$ a $B$ sú vzdialené $6\,\text{km}$, tábory $A$ a $C$ sú vzdialené $9\,\text{km}$ a uhol pri tábore $A$ je $\gamma = 70\degree$.

1. Vypočítaj vzdialenosť táborov $B$ a $C$ pomocou kosínusovej vety.
1. Horolezec musí ísť z $B$ do $C$. Je táto trasa kratšia ako $10\,\text{km}$?

**13)**

Dvaja turisti vyšli z toho istého bodu. Prvý prešiel $8\,\text{km}$, druhý $5\,\text{km}$. Uhol medzi ich trasami je $110\degree$.

1. Ako ďaleko od seba sú turisti na konci výletu?
1. Keby uhol medzi trasami bol $60\degree$ (namiesto $110\degree$), boli by turisti bližšie alebo ďalej od seba? Vypočítaj a porovnaj.

# Zmiešané úlohy

**14)**

Na turistickej mape sú zaznačené tri miesta: $A = (1, 1)$, $B = (5, 4)$, $C = (5, 1)$.

1. Vypočítaj dĺžky všetkých strán trojuholníka $ABC$.
1. Vypočítaj uhol $\alpha$ pri vrchole $A$ pomocou kosínusovej vety.
1. Overte výsledok iným spôsobom: urči vektory $\overrightarrow{AB}$ a $\overrightarrow{AC}$ a vypočítaj uhol medzi nimi cez skalárny súčin. Zhodujú sa výsledky?

**15)**

Mesto plánuje novú cyklostrádu medzi tromi bodmi: $X = (0, 0)$, $Y = (8, 0)$ a $Z = (5, 6)$ (súradnice v stovkách metrov).

1. Urči vektory $\overrightarrow{XY}$, $\overrightarrow{YZ}$ a $\overrightarrow{ZX}$.
1. Vypočítaj celkovú dĺžku cyklotrasy $X \to Y \to Z \to X$.
1. Vypočítaj uhol pri vrchole $Y$ (uhol, ktorý cyklistovi pri odbočke v $Y$).
1. Napíš rovnicu priamky, na ktorej leží úsek $XY$, a rovnicu priamky, na ktorej leží úsek $XZ$.
