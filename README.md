# grasp
Grasp is a simple cli application. 
It runs cli commands from config file 'grasp.json' (as .bat/.cmd/.sh do)

## grasp.json example for Windows (cmd)
```
{
  "tasks": [
    {
      "tool": "npm.cmd",
      "args": ["--prefix=C:\\projects\\nbuconstructorclient", "run", "prod"],
      "comment_before": "\n\tAngular build"
    },
    {
      "tool": "cmd",
      "args": ["/C", "rmdir /q /s C:\\projects\\nbuconstructor\\src\\main\\resources\\static\\"],
      "comment_before": "\n\tClean 'static'"
    },
    {
      "tool": "cmd",
      "args": ["/C", "rmdir /q /s C:\\projects\\nbuconstructor\\src\\main\\resources\\templates\\"],
      "comment_before": "\n\tClean 'index.html'"
    },
    {
      "tool": "cmd",
      "args": ["/C", "xcopy /s /e /h C:\\projects\\nbuconstructorclient\\dist\\nbuconstructor-frontend\\ C:\\projects\\nbuconstructor\\src\\main\\resources\\static\\"],
      "comment_before": "\n\tCopy 'satatic'"
    },
    {
      "tool": "cmd",
      "args": ["/C", "mkdir C:\\projects\\nbuconstructor\\src\\main\\resources\\templates\\"],
      "comment_before": "\n\tMkdir 'templates'"
    },
    {
      "tool": "cmd",
      "args": ["/C", "copy C:\\projects\\nbuconstructorclient\\dist\\nbuconstructor-frontend\\index.html C:\\projects\\nbuconstructor\\src\\main\\resources\\templates\\"],
      "comment_before": "\n\tCopy 'index.html'"
    },
    {
      "tool": "mvn.cmd",
      "args": ["--file=C:\\projects\\nbuconstructor", "clean", "install"],
      "comment_before": "\n\tMvn build"
    },
    {
      "tool": "explorer.exe",
      "args": ["C:\\projects\\nbuconstructor\\target"],
      "comment_before": "\n\tOpen target dir"
    }
  ]
}

```

## Installation
You can build grasp from sours
```
> cargo build --release
```
Put path to release dir in PATH env.
Create grasp.json and run it with 
``` 
> grasp run 
```
