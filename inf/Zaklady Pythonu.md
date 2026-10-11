---
title: Príklady Doučovanie INF -- Python Verzia
author: Michal Spano
header-includes: |
    \usepackage[utf8]{inputenc}
    \usepackage[slovak]{babel}
    \usepackage{fancyhdr}
    \usepackage{gensymb} 
    \usepackage{textcomp}
    \pagestyle{fancy}
    \fancyhead[CO,CE]{Python}
    \fancyfoot[CO,CE]{Materiály Doučovanie -- Michal Špano, © 2024---2026}
    \fancyfoot[LE,RO]{\thepage}
papersize: a4
fontsize: 12pt
colorlinks: true
geometry: margin=2cm
output: pdf_document
---

\newpage

# Úvod

Často dostávam dotazy, že programovanie je len pre istú *elitu* ľudí, takých, ktorí sedia za obrazovkou denno-denne, majú silné dioptrické okuliare a dlhé vlasy. Je to blud -- čistý stereotyp. Dúfam, že Ťa o tom presvedčím. Programovanie je určené pre *každého*, kto chce riešiť hravé úlohy. Na riešenie týchto cvičení **nepotrebuješ žiadne predošlé skúsenosti**. Čo teda budem potrebovať?

> *,,Odhodlanie, čas a vytrvalosť.''*

No to predsa nie je nič nezvládnuteľné! Programovanie je zručnosť ako každá iná: hra na klavíri, lyžovanie a či písanie básni. Čím viac človek trénuje, tým to je lepšie. Preto len pasívne čítať programovanie nefunguje -- dokážeme sa naučiť lyžovať len tým, že pozeráme ZOH v TV? \;)

Máš pocit, že niečo nesedí? To sa môže stať, nikto nie je predsa perfektný \=). Napíš mail na `<spano@michalspano.com>`. Taktiež budem rád za spätnú väzbu.

<!--
![](lenny_bexter.png){width=33%}
-->

## Pár praktických poznámok

Príklady sú zväčša zoradené tématicky od náročnosti a pre odhodlaných pridávam aj *bonus*. 

Na riešenie týchto úloh ti stačí hocijaký textový editor (`vim`, IDLE, VS (Code), dokonca i Notepad) a dostatočne nový (`>= 3.8`) `Python` spúšťač. Mnohé cvičenia môžeš skúsiť priamo v shell-y.

V cvičeniach zvyknem zaznačovať vstup `<<<` a výstup `>>>` programu takto. Ukážme si malý príklad

```txt
>>> Pridaj dve cisla.
<<< a = 2
<<< b = 3
>>> 5
```

Niekedy, najmä pri funkciach, je jednoduchšie ukázať očakávanú výstupnú hodnotu priamo v riadku, resp.

```python
faktorial(5)           # 120
je_prvocislo(7)        # True
trikrat_meno('Michal') # 'MichalMichalMichal'
```

takýto kód si môžeš prekopírovať do editora a priamo otestovať.

Taktiež si prečítaj poznámky malým za pätou strany, tie slúžia\footnote{na pomoc v riešení a ponúkajú doplnkové informácie.}

> *Veľa štastia!*

\newpage

# Základy Programovania

## Premenné, podmienky a slučky

**1)** Vytvor program, ktorý sa opýta na meno, priezvisko a rok narodenia. Úlohou
programu je potom vypísať reťazec v tvare: `priezvisko_meno-ROK`\footnote{Pripomeň si, že operácia `+` funguje i na reťazcoch. Bonus: tzv. `f-string`.}. Napr.:

```txt
<<< meno      = Andrej
<<< priezisko = Slovak
<<< rok       = 2003
>>> Andrej_Slovak-2003
```

**2)** Vytvor jednoduchú *BMI kalkulačku*. Program sa spýta na výšku a váhu. Potom
vypíše hodnotu *BMI* zaokrúhlenú na dve desatinné miesta\footnote{Hint: `round(n,k)`.} podľa vzorca: $\frac{\text{váha (kg)}}{\text{výška (m)}^2}$.

**3)** Ziači píšu písomku z informatiky, ktorá má 30 bodov. V triede je príliš veľa žiakov a pani učiteľka bude cez víkend zaneprázdnená a potrebuje pomoc s
hodnotením písomiek. (a) Pomôž jej s programom, ktorý na základe stupnice priradí
žiakovi známku od $1 \dots 5$.

| Počet bodov | Známka |
| :---------: | :----: |
| $\geq 25$   | `1`    |
| $20 \to 24$ | `2`    |
| $14 \to 19$ | `3`    |
| $9 \to 13$  | `4`    |
| inak        | `5`    |

```txt
<<< pocet bodov = 7
>>> znamka => 5
```

\noindent (b) Tvoj program vypíše správu `Chyba` ak náhodou napíšeme počet bodov vyšši ako max. počet bodov písomky.

**4)** Vytvor program, ktorý zistí či existuje trojuholník $\Delta$ s dĺžkami strán $a,b,c$. Pre každý takýto trojuholník $\Delta$ platí, že:

$$\underbrace{a + b > c}_{\text{Podmienka } \#1},\; \underbrace{a + c > b}_{\text{Podmienka } \#2},\; \underbrace{b + c > a}_{\text{Podmienka } \#3}.$$

Ako správne vidíš, musia platiť až 3 podmienky **zároveň**. Teda

$$
    \text{Podmienka } \#1 \textit{ a zároveň }
    \text{podmienka } \#2 \textit{ a zároveň }
    \text{podmienka } \#3.
$$

No ako docieliť toto v programovaní? Poznáme tzv. *logické spojky* z
matematickej logiky (ktoré celkom prirodzene vyplývajú aj z jazyka, ktorý
hovoríme, napr. Slovenčina)! Ak chceme vyjadriť, že 2 podmienky platia
**zároveň**, použijeme spojku `and` (teda po slovensky 'a' alebo 'a zároveň').
Noapak, existuje aj spojka `or` (teda 'alebo').

**Otázka**: ako by sa zmenil môj program, keby použijem `or` namiesto `and`
medzi podmienkami?

**5)** Vytvor program, ktorý simuluje jedno kolo hry *kameň-papier-nožnice* (skrátene `K`,`P`,`N`). Priebeh hry môže vyzerať napr. takto:

```txt
<<< hrac1   = K
<<< hrac2   = P
>>> vyherca = hrac2

<<< hrac1   = N
<<< hrac2   = N
>>> Remiza
```

**6)** Jedno kolo hry je celkom *nuda*. Vylepši program z úlohy 5) tak, že hra
*kameň-papier-nožnice* sa opakuje, pokiaľ jeden z hráčov nezíska aspoň $3$
body\footnote{Pravdaže, za remízu nedostáva nikto bod.}. Je očividné, že na toto budeme potrebovať slučku, no ktorú? Postačí aj `for`-cyklus? Zdôvodni.\footnote{Zamysli sa: môže byť takáto hra teoreticky nekonečná?}

```txt
<<< hrac1 = K
<<< hrac2 = P

<<< hrac1 = N
<<< hrac2 = K

<<< hrac1 = P
<<< hrac2 = N

>>> vyherca = hrac2
```

**7)** Vytvor program, ktorý zistí, či dáne číslo je prvočíslo\footnote{Číslo $p$ je prvočíslo jedine ak $p$ má dvoch delitelov, teda $1$ a $p$. Najmenšie prvočíslo
je $2$.}.

```python
je_prvocislo(2)  # True
je_prvocislo(9)  # False
je_prvocislo(17) # True
```

**8)** Vypíš pre dané $0 \leq n \leq 10$ tabuľku s *malou násobilkou*. Napr. pre $n = 7$ máme:

```txt
>>> 7 x 1  = 7
>>> 7 x 2  = 14
>>> 7 x 4  = 21
>>> ...
>>> 7 x 10 = 70
```

**9)** Vytvor program, ktorý náhodne vyberie číslicu $n$. Tento program sa pýta hráča na číslo pokiaľ neuhádne toto číslo $n$. Pre $n=2$ vyzerá hra takto

```txt
>>> Mam nahodne cislo (hra sa zacina)
>>> =================================

<<< pokus? 4
>>> Nespravne!
<<< pokus? 8
>>> Nespravne!
<<< pokus? 2
>>> Spravne.
```

Na náhodu máme v Pythone balíček `random`, ktorý pridáš do kódu cez príkaz
`import random`. Pomôž si funkciou `randint(a,b)`, ktorá vytvorí náhodné číslo
zo zatvoreného intervalu $[a,b]$, resp. aj $a,b$ vrátane. Napr.

```python
import random
r = random.randint(1,4)
# hodnota r moze byt 1, 2, 3 alebo 4.
```

**10)** Uprav program z úlohy 9) tak, že počet pokusov nie je **neobmedzený**,
teda pridaj do kódu premennú `pocet_pokusov` (s ľubovoľnou hodnotou). 
Program sa teda ukončí (i) akonáhle uhádneme náhodné číslo alebo (ii) vyčerpáme
počet pokusov. Ak vyčerpáme pokusy, program vypíše: `Vycerpal si vsetky pokusy! Koniec.`

