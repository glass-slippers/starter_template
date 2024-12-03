
`cargo tauri build` will build the csr app (it uses app/style/tailwind.css and ignores app/style/output.css), it uses trunk

run

`npx tailwindcss -i .app/style/tailwin.css -o .app/style/output.css --watch`
to update the output.css for the ssr web build, (cargo leptos is told that it uses a output.css css file), this uses cargo leptos and the `app/Cargo.toml`


### testing

We can use playwright to test our web server in our CI/CD but we can't test our tauri application this way because playwright uses devtools protocols and tauri currently only supports webdriver. So we'd have to set up seperate webdriver tests to test the unique functionality of our app or expose the app inside of a browser webview -> browser (would that lose any platform specific functionality?) either way doesn't sound great. Maybe there will be a plugin in the future.