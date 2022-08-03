

## Running Tailwind with wathc for prod
```bash
NODE_ENV=production tailwindcss -c ./tailwind.config.cjs -o ./tailwind.css --minify --watch
NODE_ENV=dev tailwindcss -c ./tailwind.config.cjs -o ./tailwind.css --minify --watch
```

