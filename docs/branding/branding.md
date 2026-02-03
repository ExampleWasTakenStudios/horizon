# Branding Guidelines

## 1. Identity

### Logo

The Horizon logo represents a stylized sunset over a horizon line.

- **Icon:** 16x16 grid optimized.
- **Gradient:** "Horizon Glow" (Orange to Gold)

<details>
  <summary>Expand for logo SVGs</summary>

```svg
<!-- TRANSPARENT BACKGROUND -->
<svg width="512" height="512" viewBox="0 0 16 16" xmlns="http://www.w3.org/2000/svg">
<defs>
  <linearGradient id="horizonGradient" x1="0%" y1="100%" x2="100%" y2="0%">
    <stop offset="0%" stop-color="#FF6A00" /> <stop offset="100%" stop-color="#FFB800" /> </linearGradient>
</defs>

<path d="M2 9.5 A6 6 0 0 1 14 9.5 Z" fill="url(#horizonGradient)" />

<rect x="1" y="10.5" width="14" height="2" rx="0.5" fill="url(#horizonGradient)" />
</svg>

<!-- COLORED BACKGROUND -->
<svg width="512" height="512" viewBox="0 0 16 16" xmlns="http://www.w3.org/2000/svg">
<defs>
  <linearGradient id="horizonGradient" x1="0%" y1="100%" x2="100%" y2="0%">
    <stop offset="0%" stop-color="#FF6A00" />
    <stop offset="100%" stop-color="#FFB800" />
  </linearGradient>
</defs>

<rect width="16" height="16" fill="#0E131B" />

<path d="M2 9.5 A6 6 0 0 1 14 9.5 Z" fill="url(#horizonGradient)" />

<rect x="1" y="10.5" width="14" height="2" rx="0.5" fill="url(#horizonGradient)" />
</svg>
```

</details>

### Color Palette

| Name           | Hex     | Usage                  |
| -------------- | ------- | ---------------------- |
| Horizon Orange | #FF6A00 | Primary Gradient Start |
| Solar Gold     | #FFB800 | Primary Gradient End   |
| Dawn Black     | #0E131B | Logo Background        |
| Black          | #000000 | Background             |
| White          | #FFFFFF | Foreground             |

## 2. Typography

**Headings:** [Satoshi](https://www.fontshare.com/fonts/satoshi)
Used for titles, large-scale data, and product names.

- **Weights:** Bold (700), Medium (500)

**Body:** [Metropolis](https://github.com/ExampleWasTakenStudios/Metropolis)
Used for content text, UI labels, and small-size numerical data.

- **Weights:** Regular (400), Medium (500)

## 3. Interface Setup

Horizon uses `shadcn/ui` with a custom configuration.

### Initialization Script

Run this to scaffold the project.

> [!NOTE]
> This script initializes the project with Geist. You must manually replace the font configuration in `tailwind.config.js` with Satoshi/Metropolis after installation.

```sh
pnpm dlx shadcn@latest create --preset "https://ui.shadcn.com/init?base=radix&style=vega&baseColor=neutral&theme=neutral&iconLibrary=lucide&font=geist&menuAccent=subtle&menuColor=default&radius=default&template=vite&rtl=false" --template vite
```
