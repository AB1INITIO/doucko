#!/usr/bin/env python3
r"""
preprocess.py — convert LaTeX macros to pandoc-native markdown syntax.

Transforms (outside code blocks and inline code spans):
  \url{URL}          ->  [URL](URL){.uri}
  \footnote{content} ->  ^[content]

Balanced-brace extraction ensures nested braces in content are handled
correctly (e.g. math inside a footnote: \footnote{see $f(x) = \{x\}$}).
"""
import sys


def extract_braced(src: str, pos: int):
    """Return (content, end_pos) for the balanced-brace group at pos ('{')."""
    assert src[pos] == "{"
    depth, i = 0, pos
    while i < len(src):
        c = src[i]
        if c == "{":
            depth += 1
        elif c == "}":
            depth -= 1
            if depth == 0:
                return src[pos + 1 : i], i + 1
        i += 1
    return None, pos  # unbalanced — leave unchanged


def preprocess(src: str) -> str:
    out = []
    i, n = 0, len(src)

    while i < n:
        # ── Fenced code block (``` or ~~~) ───────────────────────────────────
        if src[i : i + 3] in ("```", "~~~") and (i == 0 or src[i - 1] == "\n"):
            fence = src[i : i + 3]
            end = src.find("\n" + fence, i + 3)
            if end != -1:
                end = src.find("\n", end + len(fence) + 1)
                end = (end + 1) if end != -1 else n
            else:
                end = n
            out.append(src[i:end])
            i = end
            continue

        # ── Inline code span (` ... `) ───────────────────────────────────────
        if src[i] == "`":
            j = src.find("`", i + 1)
            end = (j + 1) if j != -1 else n
            out.append(src[i:end])
            i = end
            continue

        # ── \url{URL} ────────────────────────────────────────────────────────
        if src[i : i + 5] == "\\url{":
            content, end = extract_braced(src, i + 4)
            if content is not None:
                out.append(f"[{content}]({content}){{.uri}}")
                i = end
                continue

        # ── \footnote{content} ───────────────────────────────────────────────
        if src[i : i + 10] == "\\footnote{":
            content, end = extract_braced(src, i + 9)
            if content is not None:
                out.append(f"^[{content}]")
                i = end
                continue

        out.append(src[i])
        i += 1

    return "".join(out)


if __name__ == "__main__":
    sys.stdout.write(preprocess(sys.stdin.read()))
