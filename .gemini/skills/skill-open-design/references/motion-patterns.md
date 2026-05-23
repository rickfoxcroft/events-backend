# Event App Motion Patterns

Standardized animation recipes for the "Open Design" style. While these examples use `framer-motion` syntax, the principles apply to any animation library or CSS transitions.

## 🚀 Entrance Animations (Staggered)

Use for lists of events, venues, or search results.

```tsx
const containerVariants = {
  hidden: { opacity: 0 },
  visible: {
    opacity: 1,
    transition: {
      staggerChildren: 0.1,
      delayChildren: 0.2,
    },
  },
};

const itemVariants = {
  hidden: { y: 20, opacity: 0 },
  visible: {
    y: 0,
    opacity: 1,
    transition: { type: "spring", stiffness: 300, damping: 24 },
  },
};
```

## 🖱 Interactive Hover (Magnetic)

Use for high-priority CTA buttons or interactive cards.

```tsx
<motion.button
  whileHover={{ scale: 1.05, y: -2 }}
  whileTap={{ scale: 0.95 }}
  transition={{ type: "spring", stiffness: 400, damping: 10 }}
>
  View Event
</motion.button>
```

## 📜 Scroll-Linked Reveal

Use for section headings and large images.

```tsx
<motion.div
  initial={{ opacity: 0, x: -50 }}
  whileInView={{ opacity: 1, x: 0 }}
  viewport={{ once: true, margin: "-100px" }}
  transition={{ duration: 0.8, ease: "easeOut" }}
>
  <h2>Upcoming Events</h2>
</motion.div>
```

## 🔄 Page Transitions (Slide & Fade)

Standardized layout transition.

```tsx
const pageVariants = {
  initial: { opacity: 0, y: 10 },
  animate: { opacity: 1, y: 0 },
  exit: { opacity: 0, y: -10 },
};
```
