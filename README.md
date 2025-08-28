# Wordle Solver

A Wordle solver web app built with Leptos and Rust.

---

**This project is still in development**.

I have two concepts for solving Wordle:

1. **Intuitive Approach**: Start with words like "crane", "SALET", etc. Then, replace misplaced letters while keeping the correct ones in place to find the solution. This is the most intuitive approach.

2. **Elimination Approach**: Also starts with words like "crane" and "salet", but the second word should exclude the correct letters while including letters from the second, third, or possibly fourth most suitable words. This aims to eliminate more possibilities, so hopefully the third guess will solve it—or, if not, the fourth and fifth attempt definitely will.