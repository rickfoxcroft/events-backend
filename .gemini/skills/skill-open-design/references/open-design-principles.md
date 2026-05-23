# Open Design Principles (Event App Edition)

Adopted from the `nexu-io/open-design` framework, these principles ensure the website feels modern, professional, and accessible.

## 1. Local-First Development

Prioritize the local workspace as the "source of truth." Design should happen in code (Astro/Tailwind) rather than in static design tools whenever possible.

## 2. Component Autonomy

Every component (Card, Header, Footer) should be visually self-contained but follow a shared design DNA.

- Use **Slot-based architecture** for flexibility.
- Ensure components look "flawless" even with varying content lengths.

## 3. Visual Hierarchy & "Air"

- **Open Layouts:** Avoid heavy borders. Use whitespace (padding/margins) and subtle shadow depth to create separation.
- **Micro-Copy:** Typography should do the heavy lifting. Use clear, high-impact fonts for headings and readable fonts for body text.

## 4. Performance as Design

A slow site cannot be "flawless."

- Optimize all assets.
- Keep animations lightweight (prefer CSS transforms over layout shifts).

## 5. Community Inclusivity

Accessibility is paramount.

- Maintain a contrast ratio of at least 4.5:1 for all text.
- Ensure all interactive elements have visible focus states.
