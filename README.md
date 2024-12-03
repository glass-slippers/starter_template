
`cargo tauri build` will build the csr app (it uses app/style/tailwind.css and ignores app/style/output.css), it uses trunk

run

`npx tailwindcss -i .app/style/tailwin.css -o .app/style/output.css --watch`
to update the output.css for the ssr web build, (cargo leptos is told that it uses a output.css css file), this uses cargo leptos and the `app/Cargo.toml`