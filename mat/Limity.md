---
title: Príklady Doučovanie - Erika
author: Michal Spano
header-includes: |
    \usepackage{fancyhdr}
    \usepackage{gensymb} 
    \pagestyle{fancy}
    \fancyhead[CO,CE]{Príklady Doučovanie - Erika}
    \fancyfoot[CO,CE]{Materiály Doučovanie - Michal Spano, © 2025}
    \fancyfoot[LE,RO]{\thepage}
    \DeclareMathOperator{\tg}{tg}
    \DeclareMathOperator{\cotg}{cotg}
papersize: a4
fontsize: 12pt
colorlinks: true
geometry: margin=1.5cm
output: pdf_document
---

## 1. Funkcie \& Limity

Určme limity daných funkcií:

1. $\lim_{x \to 3}(2x^2 - x + 1)$
2. $\lim_{x \to \frac{\pi}{4}} \left( \sin x + \cos x \right)$
3. $\lim_{x \to 2} \cfrac{x^2 - 4}{x - 2}$
4. $\lim_{x \to -1} \cfrac{x^2 - 1}{x + 1}$
5. $\lim_{x \to 5} \cfrac{(x - 5)(x + 3)}{x - 5 + (x - 5)^2}$

Pomocou grafu zisti $\lim_{x \to 1} f(x)$ pre (*nespojitú*\footnote{Spomeň si
na príklad $f(x)=\frac{|x|}{x}$; o spojitosti viac nabudúce.}) funkciu
$f(x)=
    \begin{cases}
        2x + 1, & x < 1,\\
        4, & x = 1,     \\
        x^2 - 1, & x > 1.
    \end{cases}
$

---

*(Doplnkové príklady na zopakovanie si grafov)*

Vytvor graf na $\mathbb{R}$ pre dané funckie $f(x), \dots, q(x)$.

$$f(x) =
    \begin{cases}
    x + 2, & x < 0,\\
    1, & x = 0,\\
    -x + 2, & x > 0.
    \end{cases}
$$

$$g(x) =
    \begin{cases}
        3x, & x < 2,\\
        7, & x = 2,\\
        x^2 - 1, & x > 2.
    \end{cases}
$$


$$h(x) =
    \begin{cases}
        4 - x, & x < 1,\\
        2, & x = 1,\\
        x + 1, & x > 1.
    \end{cases}
$$

$$p(x) =
    \begin{cases}
        x^2, & x < -1,\\
        0, & x = -1,\\
        -2x, & x > -1.
    \end{cases}
$$

$$q(x) =
\begin{cases}
    \sin(x), & x < \pi,\\
    0, & x = \pi,\\
    1, & x > \pi.
\end{cases}
$$
