# [Tauri](https://tauri.app/) Updater Server (TUS)

A simple and fast API to update Tauri apps centrally.

Just for fun, it's also rust. I know that there are simpler program principles and solutions for this task, but I use [rocket](https://rocket.rs/).

### file structure
```
apps
|____app_name
     |____version.txt
     |____0.0.1
          |____windwos
                |____release.json
                |____app_name.msi.zip
                |____app_name.msi.zip.sig
```

### version.txt
```txt
0.0.1
```

### release.json
```json
{
    "url": "http://example.com/apps/example_app/0.0.1/windows/example_app_0.0.1_x64_en-US.msi.zip",
    "version": "0.0.1",
    "notes": "0.0.1 release",
    "pub_date": "2022-10-09T12:58:53+01:00",
    "signature": "app_name.msi.zip.sig file content"
}
```
I currently use it to update my own private app without any problems.

In the future I plan to design the software as dynamic as possible.
