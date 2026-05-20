# Event App Design Tokens (Tailwind 4)

These tokens bridge the "Open Design" philosophy with the Event App brand.

## 🎨 Color Palette (Tailwind 4)

Use these color mappings for consistency across all dynamic components.

| Token            | CSS Variable    | Tailwind Class                          | Usage                                         |
| :--------------- | :-------------- | :-------------------------------------- | :-------------------------------------------- |
| **Brand Primary**| `--brandPrimary`| `text-primary`, `bg-primary`            | Primary actions, accents, buttons.            |
| **Brand Navy**   | `--brandNavy`   | `bg-navy`, `border-navy`                | Backgrounds, footers, high-contrast sections. |
| **Brand White**  | `--brandWhite`  | `text-white`, `bg-white`                | Primary text, secondary backgrounds.          |
| **Brand Glow**   | N/A             | `shadow-[0_0_20px_rgba(0,0,0,0.1)]`     | Hover states for cards and badges.            |

## 🛡️ Brand Integrity & Asset Protection

- **Original Colors:** Brand assets (Logos and Sponsor logos) must be rendered in their original colors to maintain brand accuracy and respect.
- **Monochrome Exceptions:** Only use monochrome (solid white or black) versions if a dedicated asset file is provided.

## ✍️ Typography

- **Display (H1):** `text-5xl font-extrabold tracking-tight leading-tight`
- **Heading (H2/H3):** `text-3xl font-bold tracking-tight`
- **Body:** `text-base text-gray-700 dark:text-gray-300 leading-relaxed`
- **Utility (Stats/Dates):** `font-mono text-sm tracking-tighter text-primary`

## 🧊 Component Tokens

- **Cards:** `rounded-xl border border-gray-100 bg-white shadow-sm hover:shadow-md transition-all duration-300`
- **Interactive Cards:** `hover:border-primary/30 hover:-translate-y-1`
- **Buttons (Primary):** `bg-primary text-white px-6 py-3 rounded-full font-bold tracking-wider hover:bg-navy transition-colors`
- **Glassmorphism:** `bg-white/10 backdrop-blur-md border border-white/20` (use for overlays).

## 📐 Layout (The "Open" Look)

- **Section Spacing:** `py-16 md:py-24`
- **Max Width:** `max-w-7xl mx-auto px-4 sm:px-6 lg:px-8`
- **Grid Stacks:** `grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-8`
