# Branding Guidelines
# Typography
## Headings
### [Satoshi](https://www.fontshare.com/fonts/satoshi)
Used for title, large-size numerical data and product names.

### [Metropolis](https://github.com/ExampleWasTakenStudios/Metropolis)
Used for content text and small-size numerical data. 

# Interface
Interfaces use the following [ui.shadcn.com](ui.shadcn.com) script:
```sh
pnpm dlx shadcn@latest create --preset "https://ui.shadcn.com/init?base=radix&style=vega&baseColor=gray&theme=blue&iconLibrary=lucide&font=inter&menuAccent=subtle&menuColor=default&radius=default&template=vite" --template vite
```

# Logo
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
