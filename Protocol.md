# Qas | Protocol

## Config
- cache latest bundled configs as QAS_CONFIG/.cached 
- bundle all files with syntax as 

```json
"lang": "py", 
"projects": {"name": "python", "branch": "dev", "https://github.com/python/python"}
``` 
- branch field defaults to Git, that is `master`
- check if empty
- check if config has properly syntax
- `url` to URI
- `name` as lowercase and _ only

### Api 
- parse all files

## Local
- parse json files and bundle as api sintaxe
- url to uri

### Single
- bundle file as config syntax
- consumes piped input

## Front
- consume array properly processed
- run action on array
- log actions and it's output
