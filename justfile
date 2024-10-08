# just configuration for shuttle-template-vite-vanilla

# vite hot development server
dev:
  cd site/vite-vanilla && bun run dev

# build dist folder for deployment
build:
  cd site/vite-vanilla && bun run build

# shuttle local run testing
local:
  shuttle run

