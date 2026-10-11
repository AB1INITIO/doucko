---
title: Príklady Doučovanie INF -- Python Verzia -- Doplnkové Cvičenia
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

Ak si chceš overiť, že si 100% porozumel daným konceptom, tak si na správnom mieste! Tieto úlohy nie sú *povinné*, no pomôžu ti prehĺbiť porozumenie učiva a sú písané hravou formou tak, aby boli prístupné pre každého.

# Základy Programovania

## Premenné, podmienky a slučky

**1--2)** Ako som už mnohokrát spomínal, program môžeme (štandardne) vnímať ako nejakú 'funkciu' so vstupom (eng. input) a výstupom (eng. output), ktorá teda vezme vstupné hodnoty, *niečo s ními spraví* a vráti naspäť nejaký výsledok. To nie je zatiaľ nijako zaujímavé a je to i celkom logické. Pointa tohto textu je nadobudnuť u čitateľa lepšiu intuíciu o programoch, no tiež s jedným veľkým *pozor!*.

Na druhom stupni sa žiaci prvýkrát stretávajú s funkciami (i keď s funkciami sa človek stretáva už veľmi skoro bez toho aby si to uvedomil -- prečo?). Spomínaš si napríklad na lineárnu funkciu so zápisom $f: y = kx + q$? Toto je výraz, ktorý rezonuje s mnohými. No i naše programy môžeme vnímať veľmi podobne. Predstav si teda funkciu $f(x) = 2x+1$ určenú na prirodzených číslach, resp $f : \mathbb{N} \to \mathbb{N}$. Napríklad $1 \mapsto 1 \cdot 2 + 1 = 3$, $2 \mapsto 5$, atď. Toto je predsa niečo, čo vieme jednoducho vyjadriť programom!

```python
x   = int(input('x = '))
f_x = 2*x + 1
print(f_x)

# alebo ak poznáme funkcie v programovaní (viď neskoršiu kapitolu)

def f(x):
    return 2*x + 1

x = int(input('x = '))
print(f(x))
```

No je tu menší fáčik! Spomínaš si na definíciu funkcie? Je to tzv. pravidlo, ktoré priradí každému prvku (zo vstupnej množiny) **práve jeden** prvok z výstupnej množiny. Množina je len nejaká kolekcia prvkov, napr $A = \{a, b\}$. Teda napr. $g: A \to A$, kde $a \mapsto b$, $a \mapsto a$, $b \mapsto a$ nie je funkcia, pretože $a$ môže byť priradené k $a$ alebo $b$\footnote{Toto je tzv. 'relácia' v matematike.}. No toto sa dá vcelku ľahko docieliť v programovaní. *Prečo?* To sa dozvieš neskôr pri príkladoch, no napr. náhoda alebo čítanie zo súborov nám dopomôže.

**Otázka:** myslíš si, že každá funkcia z matematiky sa dá prepísať do programovania (napr. v jazyku Python)? Platí to aj naopak? Vytvor príklady a skús s nimi zdôvodniť tvoju odpoveď.
