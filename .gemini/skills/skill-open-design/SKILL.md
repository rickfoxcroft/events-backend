---
name: skill-open-design
description: Event App "Open Design" System. Use this skill when designing UI components, implementing animations, or auditing the visual polish of the Event App website. It applies "Open Design" principles using Tailwind 4 and modern frontend practices.
---

# Event App: Open Design

This skill transforms Gemini CLI into a specialized Design Engineer for the Event App. It implements modern "Open Design" workflows to ensure a high-quality, professional UI.

## 🧭 Core Directives

1.  **Always Reference Tokens:** Never hardcode HEX values. Refer to [design-tokens.md](references/design-tokens.md) for color and typography mappings.
2.  **Energy via Motion:** Every interactive element should have motion or transition states. See [motion-patterns.md](references/motion-patterns.md) for specific recipes.
3.  **The "Open" Look:** Prioritize whitespace, fluid layouts, and typography over rigid containers. Review [open-design-principles.md](references/open-design-principles.md).
4.  **Auditing Mandates:** Before finishing any UI task, check against the `GEMINI.md` mandates.

## 🛠 Usage Patterns

### Creating a New Component

When asked to "Build an event card" or "Create a venue list":

1.  **Research:** Check `references/design-tokens.md` for the appropriate card/button styles.
2.  **Implement:** Use Tailwind 4 utility classes for layout.
3.  **Animate:** Apply an entrance or hover transition from `references/motion-patterns.md`.
4.  **Verify:** Ensure proper component structure and accessibility.

### Auditing Visual Polish

When asked to "Make this page look better" or "Add more dynamic feel":

- Add staggered animations to existing lists.
- Refine typography tracking on headings.
- Replace hard borders with soft shadows or subtle gradients.

## 🗂 Resources

- **[Design Tokens](references/design-tokens.md)**: Colors, Typography, and Component UI specs.
- **[Motion Patterns](references/motion-patterns.md)**: Animation recipes for high-energy interactions.
- **[Open Design Principles](references/open-design-principles.md)**: Philosophy for modern, fluid web design.
