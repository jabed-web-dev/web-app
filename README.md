# web-app
### A cli web app created by Tauri app

> **To build this app, the source code needs to be updated.**\
> **All the code is in `tauri-app_mod.rs` with information.**

### Run and Build this app
`cargo run -- --url http://localhost:5000 -w 400 -w 740`\
`cargo build --release`

\
**These options are there.**
```pwsh
:long      :short       :default
--url          -u       http://localhost:5000
--title        -t       Web App
--width        -w       800
--height       -h       600
--left         -x       center
--top          -y       center
--light        -l       system
--dark         -d       system
--ontop        -ot      false
--devtools     -dt      false
--fullscreen   -fs      false
```

> `--left` and `--top` work with negative numbers (-1) to indicate `right` and `bottom`.
